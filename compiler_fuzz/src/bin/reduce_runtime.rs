use compiler_fuzz::generate::get_generator_flags;
use compiler_fuzz::reduction::{check_for_ub, compile_clean_code};
use compiler_fuzz::{ignorable_warnings, FailInfo, Runner, RuntimeFailInfo, RuntimeFailType};
use std::fs::File;
use std::io::Read;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::{env, process::exit};
use xshell::{cmd, Shell};

fn main() -> anyhow::Result<()> {
    let working_dir = env::current_dir()?;

    let reduction_dir = env::var("REDUCTION_DIR")?;
    let reduction_path = PathBuf::from(reduction_dir);

    let mut file = File::open(reduction_path.join("fail_info.yaml")).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let fail_info: FailInfo = serde_yaml::from_str(&data).unwrap();

    let testcases = match &fail_info {
        FailInfo::Ice(ice_info) => &ice_info.testcases.clone(),
        FailInfo::Execution(exec_info) => &exec_info.testcase.clone(),
        FailInfo::Runtime(runtime_info) => &runtime_info.testcase.clone(),
    };

    println!("Testcases {:?}", testcases);

    for testcase in testcases {
        assert!(reduction_path.join(testcase).exists())
    }

    if testcases.len() > 1 {
        let _ = (0..testcases.len() + 1).map(|i| {
            assert!(working_dir
                .join(format!("fast_reducible_compiler_opts_{i}.txt"))
                .exists());
            assert!(working_dir
                .join(format!("slow_reducible_compiler_opts_{i}.txt"))
                .exists());
        });
    } else {
        assert!(working_dir
            .join("fast_reducible_compiler_opts.txt")
            .exists());
        assert!(working_dir
            .join("slow_reducible_compiler_opts.txt")
            .exists());
    }

    match fail_info {
        FailInfo::Runtime(fail_info) => produce_fail(&fail_info, &working_dir)?,
        FailInfo::Ice(_) => unreachable!(),
        FailInfo::Execution(_) => unreachable!(),
    }

    Ok(())
}

fn execute_code(
    sh: &Shell,
    compilers: &Vec<PathBuf>,
    testcases: &Vec<PathBuf>,
    runner: &Runner,
    file_prefix: &str,
    compiler_flags: &Vec<Vec<String>>,
    ignorable_warnings: &Vec<String>,
    fail_type: &Option<RuntimeFailType>,
) -> anyhow::Result<String> {
    compile_clean_code(
        sh,
        compilers,
        testcases,
        compiler_flags,
        PathBuf::from(format!("{file_prefix}_testcase.o")),
        ignorable_warnings,
    )?;

    let mut run_command: std::process::Command = match &runner {
        Runner::Native => cmd!(sh, "timeout -k 0.1 2 ./{file_prefix}_testcase.o 1")
            .quiet()
            .into(),
        Runner::Qemu(qemu_config) => {
            let qemu_cpu = match &qemu_config.cpu_flags {
                compiler_fuzz::RunnerArguments::Fixed(flags) => flags,
                compiler_fuzz::RunnerArguments::Generated(generation_script) => &cmd!(
                    sh,
                    "{generation_script} --elf-file-path {file_prefix}_testcase.o --print-qemu-cpu"
                )
                .read()?,
            };

            let qemu = if qemu_cpu.starts_with("rv32") {
                &qemu_config.rv32path
            } else {
                &qemu_config.rv64path
            };

            cmd!(sh, "timeout -k 0.1 2 {qemu} {file_prefix}_testcase.o 1")
                .env("QEMU_CPU", qemu_cpu)
                .quiet()
                .into()
        }
    };

    println!("Run cmd: {:?}", run_command);

    let command_output = run_command.output()?;

    let stdout = String::from_utf8(command_output.stdout)?;
    let stderr = String::from_utf8(command_output.stderr)?;
    let signal = command_output.status.signal().unwrap_or(0).to_string();
    sh.write_file(file_prefix.to_owned() + "_exec_stdout.txt", &stdout)?;
    sh.write_file(file_prefix.to_owned() + "_exec_stderr.txt", &stderr)?;
    sh.write_file(file_prefix.to_owned() + "_exec_signal.txt", &signal)?;

    match command_output.status.code() {
        Some(0) => match fail_type {
            Some(RuntimeFailType::Mismatch) => Ok(stdout),
            None => {
                // Unknown, any fail will do
                Ok(stdout)
            }
        },
        Some(1) => {
            println!("Unexpected execution fail");
            exit(1);
        }
        Some(124) => {
            // Timeout
            println!("Exec Timeout!");
            exit(1);
        }
        None => {
            match command_output.status.signal() {
                Some(9) => {
                    // Killed by timeout -k
                    println!("Exec Timeout!");
                    exit(1);
                }
                Some(4) => {
                    println!("Unexpected illegal insn");
                    exit(1);
                }
                Some(i) => {
                    // Killed by timeout -k
                    println!("Unknown exec signal: {i}");
                    exit(1);
                }
                None => unreachable!("If the exit code is None, the signal must be set!"),
            }
        }
        Some(_) => {
            println!("Unrecognized compilation failure: {}", stderr);
            exit(1);
        }
    }
}

fn produce_fail(fail_info: &RuntimeFailInfo, working_dir: &PathBuf) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(working_dir);

    // Remove existing binaries
    let artifacts = vec![
        PathBuf::from(format!("fast_testcase.o")),
        PathBuf::from(format!("slow_testcase.o")),
    ];
    for artifact in artifacts {
        sh.remove_path(artifact)?;
    }

    if fail_info.fast_compiler.len() == 1 && fail_info.fast_compiler.len() == 1 {
        assert!(fail_info.testcase.len() == 1);

        let mut file = File::open(working_dir.join("slow_reducible_compiler_opts.txt")).unwrap();
        let mut compiler_flags_str_lines = String::new();
        file.read_to_string(&mut compiler_flags_str_lines).unwrap();

        // Ensure formatting remains
        assert!(
            compiler_flags_str_lines.contains("\n"),
            "Flags should be split up to make it easy for creduce!"
        );

        let compiler_flags_str = compiler_flags_str_lines.replace("\n\n", " ");
        let compiler_flags_str = compiler_flags_str.replace("\n", "");

        let rv32 = compiler_flags_str.contains("-march=rv32");
        check_for_ub(&sh, &fail_info.generator, &fail_info.testcase, rv32)?;

        let rv32_flags = if rv32 { "-m32 -malign-double" } else { "" };

        let fast_compiler_flags = format!("-O1 {rv32_flags}");
        let fast_compiler_flags = fast_compiler_flags
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let fast_stdout = execute_code(
            &sh,
            &fail_info.fast_compiler,
            &fail_info.testcase,
            &fail_info.fast_runner,
            "fast",
            &vec![fast_compiler_flags],
            &vec!["-w".to_string()],
            &fail_info.fail_type,
        )?;

        let slow_compiler_flags = compiler_flags_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let ignorable_warnings = ignorable_warnings();

        let slow_stdout = execute_code(
            &sh,
            &fail_info.slow_compiler,
            &fail_info.testcase,
            &fail_info.slow_runner,
            "slow",
            &vec![slow_compiler_flags],
            &ignorable_warnings,
            &fail_info.fail_type,
        )?;

        println!("fast: {fast_stdout}\nslow: {slow_stdout}");

        match fail_info.fail_type {
            Some(RuntimeFailType::Mismatch) => {
                if fast_stdout != slow_stdout {
                    println!("Success");
                    exit(0);
                } else {
                    println!("Failure: No mismatch found");
                    exit(1);
                }
            }
            None => {
                // Unknown, any error will do
                if fast_stdout != slow_stdout {
                    println!("Success");
                    exit(0);
                } else {
                    println!("Failure: No mismatch found");
                    exit(1);
                }
            }
        }
    } else {
        // Multiple
        assert!(fail_info.fast_compiler.len() > 1 && fail_info.fast_compiler.len() > 1);

        let testcase_paths = &fail_info.testcase;
        assert!(testcase_paths.len() > 1);
        assert!(
		fail_info.fast_compiler.len() == testcase_paths.len() + 1,
		    "Must be exactly n+1 compilers specified. The last compiler is used when linking all object files together."
		);
        assert!(
		fail_info.slow_compiler.len() == testcase_paths.len() + 1,
			"Must be exactly n+1 compilers specified. The last compiler is used when linking all object files together."
		);

        let fast_flags = (0..fail_info.testcase.len() + 1)
            .map(|i| PathBuf::from(format!("fast_reducible_compiler_opts_{i}.txt")))
            .collect::<Vec<_>>();
        assert!(
		fast_flags.len() == testcase_paths.len() + 1,
		"Must be exactly n+1 sets of flags specified. The last set is used when linking all object files together."
	    );

        let slow_flags = (0..fail_info.testcase.len() + 1)
            .map(|i| PathBuf::from(format!("slow_reducible_compiler_opts_{i}.txt")))
            .collect::<Vec<_>>();
        assert!(
		slow_flags.len() == testcase_paths.len() + 1,
		"Must be exactly n+1 sets of flags specified. The last set is used when linking all object files together."
	    );

        // Validate and extract flags
        let fast_flags = fast_flags
            .iter()
            .map(|flag_file| {
                let mut file = File::open(working_dir.join(flag_file)).unwrap();
                let mut compiler_flags_str_lines = String::new();
                file.read_to_string(&mut compiler_flags_str_lines).unwrap();

                // Ensure formatting remains
                assert!(
                    compiler_flags_str_lines.contains("\n"),
                    "Flags in {flag_file:?} should be split up to make it easy for creduce!"
                );

                let compiler_flags_str = compiler_flags_str_lines.replace("\n\n", " ");
                let compiler_flags_str = compiler_flags_str.replace("\n", "");
                compiler_flags_str
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<_>>();

        let slow_flags = slow_flags
            .iter()
            .map(|flag_file| {
                let mut file = File::open(working_dir.join(flag_file)).unwrap();
                let mut compiler_flags_str_lines = String::new();
                file.read_to_string(&mut compiler_flags_str_lines).unwrap();

                // Ensure formatting remains
                assert!(
                    compiler_flags_str_lines.contains("\n"),
                    "Flags in {flag_file:?} should be split up to make it easy for creduce!"
                );

                let compiler_flags_str = compiler_flags_str_lines.replace("\n\n", " ");
                let compiler_flags_str = compiler_flags_str.replace("\n", "");

                let slow_generator_flags =
                    get_generator_flags(&fail_info.generator, &fail_info.slow_architecture)
                        .unwrap();

                compiler_flags_str
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .chain(slow_generator_flags)
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<_>>();

        let rv32 = slow_flags
            .iter()
            .any(|x| x.iter().any(|s| s.contains("-march=rv32")));
        // check_for_ub(&sh, &fail_info.generator, &fail_info.testcase, rv32)?;

        let rv32_flags = if rv32 { "-m32 -malign-double" } else { "" };

        let fast_generator_flags =
            get_generator_flags(&fail_info.generator, &fail_info.fast_architecture)?;

        let fast_compiler_flags = format!("-O1 {}", fast_generator_flags.join(" "));
        let fast_compiler_flags = fast_compiler_flags
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let fast_compiler_flags = &(0..testcase_paths.len() + 1)
            .map(|_| fast_compiler_flags.clone())
            .collect();

        let fast_stdout = execute_code(
            &sh,
            &fail_info.fast_compiler,
            &fail_info.testcase,
            &fail_info.fast_runner,
            "fast",
            fast_compiler_flags,
            &vec!["-w".to_string()],
            &fail_info.fail_type,
        )?;
        let ignorable_warnings = ignorable_warnings();

        let slow_stdout = execute_code(
            &sh,
            &fail_info.slow_compiler,
            &fail_info.testcase,
            &fail_info.slow_runner,
            "slow",
            &slow_flags,
            &ignorable_warnings,
            &fail_info.fail_type,
        )?;

        println!("fast: {fast_stdout}\nslow: {slow_stdout}");

        match fail_info.fail_type {
            Some(RuntimeFailType::Mismatch) => {
                if fast_stdout != slow_stdout {
                    println!("Success");
                    exit(0);
                } else {
                    println!("Failure: No mismatch found");
                    exit(1);
                }
            }
            None => {
                // Unknown, any error will do
                if fast_stdout != slow_stdout {
                    println!("Success");
                    exit(0);
                } else {
                    println!("Failure: No mismatch found");
                    exit(1);
                }
            }
        }
    }
}
