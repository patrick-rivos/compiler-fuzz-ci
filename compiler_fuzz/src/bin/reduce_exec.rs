use compiler_fuzz::reduction::check_for_ub;
use compiler_fuzz::{
    ignorable_warnings, ExecFailInfo, ExecFailType, FailInfo, QemuFailType, Runner,
};
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

    for testcase in testcases {
        assert!(reduction_path.join(testcase).exists())
    }

    if testcases.len() > 1 {
        let _ = (0..testcases.len() + 1).map(|i| {
            assert!(working_dir
                .join(format!("reducible_compiler_opts_{i}.txt"))
                .exists())
        });
    } else {
        assert!(working_dir.join("reducible_compiler_opts.txt").exists());
    }

    match fail_info {
        FailInfo::Execution(fail_info) => produce_fail(&fail_info, &working_dir)?,
        FailInfo::Ice(_) => unreachable!(),
        FailInfo::Runtime(_) => unreachable!(),
    }

    Ok(())
}

fn produce_fail(fail_info: &ExecFailInfo, working_dir: &PathBuf) -> anyhow::Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(working_dir);

    let compilers = &fail_info.compiler;

    if compilers.len() == 1 {
        let mut file = File::open(working_dir.join("reducible_compiler_opts.txt")).unwrap();
        let mut compiler_flags_str_lines = String::new();
        file.read_to_string(&mut compiler_flags_str_lines).unwrap();

        // Ensure formatting remains
        assert!(
            compiler_flags_str_lines.contains("\n"),
            "Flags should be split up to make it easy for creduce!"
        );

        let compiler_flags_str = compiler_flags_str_lines.replace("\n\n", " ");
        let compiler_flags_str = compiler_flags_str.replace("\n", "");
        let compiler_flags = compiler_flags_str.split_whitespace();

        let rv32 = compiler_flags_str.contains("-march=rv32");
        check_for_ub(&sh, &fail_info.generator, &fail_info.testcase, rv32)?;

        let compiler = &compilers[0];

        let ignorable_warnings = ignorable_warnings();

        let mut compile_command: std::process::Command = cmd!(
        sh,
		"{compiler} {compiler_flags...} red.c -fsigned-char -fno-strict-aliasing -fwrapv -Wall -Wformat {ignorable_warnings...} -o testcase.o"
	)
	.into();

        let command_output = compile_command.output()?;

        let stdout = &String::from_utf8(command_output.stdout)?;
        let stderr = String::from_utf8(command_output.stderr)?;
        sh.write_file("comp_stdout.txt", stdout)?;
        sh.write_file("comp_stderr.txt", &stderr)?;

        match command_output.status.code().unwrap() {
            0 => {
                // Compilation is expected to pass
                // Make sure there aren't warnings
                if stderr.contains("warning:") {
                    println!("Unexpected warning! {stderr}");
                    exit(1);
                }
            }
            124 => {
                println!("Timeout!");
                exit(1);
            }
            _ => {
                println!("Unrecognized compilation failure: {}", stderr);
                exit(1);
            }
        };

        let mut run_command: std::process::Command = match &fail_info.runner {
            Runner::Native => cmd!(sh, "timeout -k 0.1 2 testcase.o 1").quiet().into(),
            Runner::Qemu(qemu_config) => {
                let qemu_cpu = match &qemu_config.cpu_flags {
                    compiler_fuzz::RunnerArguments::Fixed(flags) => flags,
                    compiler_fuzz::RunnerArguments::Generated(generation_script) => &cmd!(
                        sh,
                        "{generation_script} --elf-file-path testcase.o --print-qemu-cpu"
                    )
                    .read()?,
                };

                println!("{}", qemu_cpu);

                let qemu = if qemu_cpu.starts_with("rv32") {
                    &qemu_config.rv32path
                } else {
                    &qemu_config.rv64path
                };

                cmd!(sh, "timeout -k 0.1 2 {qemu} testcase.o 1")
                    .env("QEMU_CPU", qemu_cpu)
                    .quiet()
                    .into()
            }
        };

        let command_output = run_command.output()?;

        let stdout = String::from_utf8(command_output.stdout)?;
        let stderr = String::from_utf8(command_output.stderr)?;
        let signal = command_output.status.signal().unwrap_or(0).to_string();
        sh.write_file("exec_stdout.txt", stdout)?;
        sh.write_file("exec_stderr.txt", &stderr)?;
        sh.write_file("exec_signal.txt", &signal)?;

        match command_output.status.code() {
            Some(0) => {
                // This shouldn't pass!
                println!("Unexpected execution pass");
                exit(1);
            }
            Some(1) => match fail_info.fail_type {
                ExecFailType::Qemu(Some(QemuFailType::ErrorMsg)) => {
                    if stderr.contains("qemu-riscv") {
                        println!("Success");
                        return Ok(());
                    }
                }
                ExecFailType::Qemu(None) => {
                    // Has not been categorized, any fail will do
                    println!("Uncategorized exit code 1. Stderr:\n{stderr}");
                    return Ok(());
                }
                ExecFailType::Native(_)
                | ExecFailType::Qemu(Some(QemuFailType::IllegalInsn))
                | ExecFailType::Qemu(Some(QemuFailType::Segfault)) => {
                    println!("Unexpected exec exit code 1 stderr: {stderr}");
                    exit(1)
                }
            },
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
                    Some(4) => match fail_info.fail_type {
                        ExecFailType::Qemu(Some(QemuFailType::IllegalInsn)) => {
                            println!("Success");
                            return Ok(());
                        }
                        ExecFailType::Qemu(None) => {
                            // Has not been categorized, any fail will do
                            println!("Uncategorized illegal insn");
                            return Ok(());
                        }
                        ExecFailType::Native(_)
                        | ExecFailType::Qemu(Some(QemuFailType::ErrorMsg))
                        | ExecFailType::Qemu(Some(QemuFailType::Segfault)) => {
                            println!("Unexpected illegal insn");
                            exit(1)
                        }
                    },
                    Some(11) => match fail_info.fail_type {
                        ExecFailType::Qemu(Some(QemuFailType::Segfault)) => {
                            println!("Success");
                            return Ok(());
                        }
                        ExecFailType::Qemu(None) => {
                            // Has not been categorized, any fail will do
                            println!("Uncategorized segfault");
                            return Ok(());
                        }
                        ExecFailType::Native(_)
                        | ExecFailType::Qemu(Some(QemuFailType::ErrorMsg))
                        | ExecFailType::Qemu(Some(QemuFailType::IllegalInsn)) => {
                            println!("Unexpected segfault");
                            exit(1)
                        }
                    },
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
        println!("Unrecognized failure: {}", stderr);
        exit(1);
    } else {
        let testcase_paths = &fail_info.testcase;
        let flags = (0..fail_info.testcase.len() + 1)
            .map(|i| PathBuf::from(format!("reducible_compiler_opts_{i}.txt")))
            .collect::<Vec<_>>();
        assert!(compilers.len() > 1);
        assert!(testcase_paths.len() > 1);
        assert!(
		flags.len() == testcase_paths.len() + 1,
		"Must be exactly n+1 sets of flags specified. The last set is used when linking all object files together."
	    );
        assert!(
		    compilers.len() == testcase_paths.len() + 1,
		    "Must be exactly n+1 compilers specified. The last compiler is used when linking all object files together."
		);

        // Validate and extract flags
        let flags = flags
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

        let rv32 = flags
            .iter()
            .any(|x| x.iter().any(|s| s.contains("-march=rv32")));
        check_for_ub(&sh, &fail_info.generator, &fail_info.testcase, rv32)?;

        todo!();
    }
}
