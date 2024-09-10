use argh::FromArgs;
use compiler_flags_gen::Action;
use compiler_fuzz::generate::get_generator_flags;
use compiler_fuzz::{
    ignorable_warnings, ExecFailInfo, ExecFailType, FailInfo, FuzzGenerator, GccFailType,
    IceFailInfo, IceFailType, LlvmFailType, QemuFailType, Runner, RuntimeFailInfo, RuntimeFailType,
};
use env_logger::Env;
use std::env;
use std::fs::{self, File};
use std::io::Read;
use std::iter::once;
use std::path::{Path, PathBuf};
use xshell::{cmd, Shell};

#[derive(FromArgs)]
#[argh(description = "Reduce failure
Use RUST_LOG=off to turn off logging")]
struct FuzzArgs {
    /// directory containing the dump produced by fuzz.rs
    #[argh(positional)]
    fail_directory: PathBuf,

    #[argh(positional)]
    reduction_directory: PathBuf,

    /// resume an existing reduction
    #[argh(switch, short = 'r')]
    resume_existing: bool,

    /// skip c reduction
    #[argh(switch, short = 's')]
    skip_c: bool,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: FuzzArgs = argh::from_env();

    assert_ne!(
        args.fail_directory, args.reduction_directory,
        "Cannot reduce in the same directory as the fail!"
    );

    // Create reduction directory
    fs::create_dir_all(&args.reduction_directory).unwrap();
    let reduction_dir = &fs::canonicalize(&args.reduction_directory).unwrap();
    let fail_dir = &fs::canonicalize(&args.fail_directory).unwrap();

    assert!(
        !reduction_dir.exists()
            || reduction_dir.read_dir()?.next().is_none()
            || args.resume_existing,
        "Reduction directory({:?}) already has files!",
        reduction_dir
    );

    fs::create_dir_all(reduction_dir).unwrap();

    // Reduce
    let sh = Shell::new()?;
    sh.change_dir(reduction_dir);

    if !args.resume_existing {
        cmd!(sh, "cp -r {fail_dir}/. {reduction_dir}").run()?;
    }

    let mut file = File::open(reduction_dir.join("fail_info.yaml")).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let mut fail_info: FailInfo = serde_yaml::from_str(&data).unwrap();

    if args.resume_existing {
        for testcase in match &fail_info {
            FailInfo::Ice(ice_info) => ice_info.testcases.clone(),
            FailInfo::Execution(exec_info) => exec_info.testcase.clone(),
            FailInfo::Runtime(runtime_info) => runtime_info.testcase.clone(),
        } {
            assert!(
                &args
                    .reduction_directory
                    .join(testcase.to_str().unwrap().to_string() + ".orig")
                    .exists(),
                "testcase: {} doesn't exist! Start a fresh reduction",
                testcase.to_str().unwrap().to_string() + ".orig"
            )
        }
    }

    let testcases = reduce(
        &sh,
        &mut fail_info,
        reduction_dir,
        args.resume_existing,
        args.skip_c,
    )?;

    // Reload fail info
    let mut file = File::open(reduction_dir.join("fail_info.yaml")).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let fail_info: FailInfo = serde_yaml::from_str(&data).unwrap();

    report(&sh, testcases, reduction_dir, fail_info)?;

    Ok(())
}

// TODO: Add reduction config that allows the user to define a mapping from
// failure compiler to a new path.
fn reduce(
    sh: &Shell,
    fail_info: &mut FailInfo,
    reduction_dir: &PathBuf,
    resume_existing: bool,
    skip_c: bool,
) -> anyhow::Result<Vec<PathBuf>> {
    let testcases = &if !resume_existing {
        preprocess(sh, reduction_dir, fail_info)?
    } else {
        match &fail_info {
            FailInfo::Ice(ice_info) => ice_info.testcases.clone(),
            FailInfo::Execution(exec_info) => exec_info.testcase.clone(),
            FailInfo::Runtime(runtime_info) => runtime_info.testcase.clone(),
        }
    };

    for testcase in testcases {
        assert!(reduction_dir.join(testcase).exists());
    }

    match fail_info {
        FailInfo::Ice(ice_fail_info) => {
            ice_fail_info.testcases = testcases.to_vec();
        }
        FailInfo::Execution(exec_fail_info) => {
            exec_fail_info.testcase = testcases.to_vec();
        }
        FailInfo::Runtime(runtime_fail_info) => {
            runtime_fail_info.testcase = testcases.to_vec();
        }
    }
    sh.write_file("fail_info.yaml", serde_yaml::to_string(&fail_info).unwrap())?;

    // Now we can reduce it using reduce-ice
    let current_exe = env::current_exe()?;
    let current_dir = current_exe.parent().unwrap();

    // Get additional files used by generator
    let additional_files = &match match fail_info {
        FailInfo::Ice(ice_fail_info) => ice_fail_info.generator.clone(),
        FailInfo::Execution(exec_fail_info) => exec_fail_info.generator.clone(),
        FailInfo::Runtime(runtime_fail_info) => runtime_fail_info.generator.clone(),
    } {
        FuzzGenerator::Csmith(_) => vec![],
        FuzzGenerator::Yarpgen(_) => vec!["init.h".to_string()],
        FuzzGenerator::Rustsmith(_) => vec![],
        FuzzGenerator::Fixed(_) => vec![],
    };

    match fail_info {
        FailInfo::Ice(ice_fail_info) => {
            let reduce_ice = current_dir.join("reduce_ice");

            println!("Test run (with REDUCTION_DIR={:?}):", reduction_dir);

            let reducible_compiler_opts = &if testcases.len() == 1 {
                vec!["reducible_compiler_opts.txt".to_string()]
            } else {
                (0..testcases.len() + 1)
                    .map(|i| format!("reducible_compiler_opts_{i}.txt"))
                    .collect()
            };

            let _env_var = sh.push_env("REDUCTION_DIR", reduction_dir);

            cmd!(
                sh,
                "{reduce_ice} {testcases...} {reducible_compiler_opts...}"
            )
            .run()?;

            ice_fail_info.fail_type = categorize_ice_fail(sh, reduction_dir, ice_fail_info)?;
            sh.write_file("fail_info.yaml", serde_yaml::to_string(&fail_info).unwrap())?;

            cmd!(
                sh,
                "{reduce_ice} {testcases...} {reducible_compiler_opts...} {additional_files...}"
            )
            .run()?;

            if !skip_c {
                cmd!(
                    sh,
                    "creduce --timeout 10 {reduce_ice} {testcases...} {reducible_compiler_opts...} {additional_files...}"
                )
                .run()?;
            }
        }
        FailInfo::Execution(exec_fail_info) => {
            let reduce_exec = current_dir.join("reduce_exec");

            println!("Test run (with REDUCTION_DIR={:?}):", reduction_dir);

            let _env_var = sh.push_env("REDUCTION_DIR", reduction_dir);

            cmd!(sh, "{reduce_exec} {testcases...} reducible_compiler_opts").run()?;

            exec_fail_info.fail_type = categorize_exec_fail(sh, reduction_dir, exec_fail_info)?;
            sh.write_file("fail_info.yaml", serde_yaml::to_string(&fail_info).unwrap())?;

            cmd!(
                sh,
                "{reduce_exec} {testcases...} reducible_compiler_opts.txt {additional_files...}"
            )
            .run()?;

            if !skip_c {
                cmd!(
                    sh,
                    "creduce --timeout 10 {reduce_exec} {testcases...} reducible_compiler_opts.txt {additional_files...}"
                )
                .run()?;
            }
        }
        FailInfo::Runtime(runtime_fail_info) => {
            let reduce_runtime = current_dir.join("reduce_runtime");

            println!("Test run (with REDUCTION_DIR={:?}):", reduction_dir);

            let fast_reducible_compiler_opts = &if testcases.len() == 1 {
                vec!["fast_reducible_compiler_opts.txt".to_string()]
            } else {
                (0..testcases.len() + 1)
                    .map(|i| format!("fast_reducible_compiler_opts_{i}.txt"))
                    .collect()
            };
            let slow_reducible_compiler_opts = &if testcases.len() == 1 {
                vec!["slow_reducible_compiler_opts.txt".to_string()]
            } else {
                (0..testcases.len() + 1)
                    .map(|i| format!("slow_reducible_compiler_opts_{i}.txt"))
                    .collect()
            };

            let _env_var = sh.push_env("REDUCTION_DIR", reduction_dir);

            cmd!(
                sh,
                "{reduce_runtime} {testcases...} {fast_reducible_compiler_opts...} {slow_reducible_compiler_opts...}"
            )
            .run()?;

            runtime_fail_info.fail_type = Some(categorize_runtime_fail(
                sh,
                reduction_dir,
                runtime_fail_info,
            )?);
            sh.write_file("fail_info.yaml", serde_yaml::to_string(&fail_info).unwrap())?;

            cmd!(
                sh,
                "{reduce_runtime} {testcases...} {fast_reducible_compiler_opts...} {slow_reducible_compiler_opts...} {additional_files...}"
            )
            .run()?;

            if !skip_c {
                cmd!(
                    sh,
                    "creduce --timeout 10 {reduce_runtime} {testcases...} {fast_reducible_compiler_opts...} {slow_reducible_compiler_opts...} {additional_files...}"
                )
                .run()?;
            }
        }
    }

    Ok(testcases.to_owned())
}

fn report(
    sh: &Shell,
    testcases: Vec<PathBuf>,
    reduction_dir: &Path,
    fail_info: FailInfo,
) -> anyhow::Result<()> {
    // Reconstruct compiler opts
    let reduced_opts = &if testcases.len() == 1 {
        let split_opts = sh.read_file(reduction_dir.join("reducible_compiler_opts.txt"))?;
        let reduced_opts = split_opts.replace("\n\n", " ");
        let reduced_opts = reduced_opts.replace("\n", "");
        sh.write_file(
            reduction_dir.join("reduced_compiler_opts.txt"),
            &reduced_opts,
        )?;
        vec![reduced_opts]
    } else {
        let reduced_opts: Vec<String> = (0..testcases.len() + 1)
            .map(|i| {
                let split_opts = sh
                    .read_file(reduction_dir.join(format!("reducible_compiler_opts_{i}.txt")))
                    .unwrap();
                let reduced_opts = split_opts.replace("\n\n", " ");
                reduced_opts.replace("\n", "")
            })
            .collect();
        let _ = reduced_opts.iter().enumerate().map(|(i, flags)| {
            sh.write_file(
                reduction_dir.join(format!("reduced_compiler_opts_{i}.txt")),
                flags,
            )
            .unwrap();
        });
        reduced_opts
    };

    match fail_info {
        FailInfo::Ice(fail_info) => {
            let assemble_link_flags = match fail_info.action {
                Action::Compile => "-S -o /dev/null",
                Action::Assemble => "-c -o /dev/null",
                Action::Link | Action::Execute => "-o testcase.o",
            };

            let compile_commands = if testcases.len() == 1 {
                vec![format!(
                    "{} {} -Wall {} {assemble_link_flags}",
                    fail_info.compilers.last().unwrap().to_str().unwrap(),
                    reduced_opts.last().unwrap(),
                    testcases.last().unwrap().to_str().unwrap()
                )]
            } else {
                assert_eq!(fail_info.compilers.len(), testcases.len() + 1);
                assert_eq!(reduced_opts.len(), testcases.len() + 1);

                let mut compile_commands = testcases
                    .iter()
                    .zip(fail_info.compilers.clone())
                    .zip(reduced_opts.clone())
                    .map(|((t, c), f)| {
                        format!(
                            "{} {f} -Wall {} -c -o {}",
                            c.to_str().unwrap(),
                            t.to_str().unwrap(),
                            t.with_extension("o").to_str().unwrap()
                        )
                    })
                    .collect::<Vec<_>>();

                compile_commands.extend(once(format!(
                    "{} {} -Wall {} {assemble_link_flags}",
                    fail_info.compilers.last().unwrap().to_str().unwrap(),
                    reduced_opts.last().unwrap(),
                    testcases
                        .iter()
                        .map(|t| { t.with_extension("o").to_str().unwrap().to_string() })
                        .collect::<Vec<_>>()
                        .join(" ")
                )));

                compile_commands
            };

            let reproduce_sh = format!("#!/bin/bash\n{}\n", compile_commands.join("\n"));
            sh.write_file(reduction_dir.join("reproduce.sh"), reproduce_sh)?;
            cmd!(sh, "chmod +x reproduce.sh").run()?;

            match fail_info.fail_type {
                IceFailType::Llvm(Some(LlvmFailType::Llc)) => {
                    // We can reduce llvm further
                    extract_llvm_ir_llc(sh, reduction_dir, &fail_info)?;
                }
                IceFailType::Llvm(Some(LlvmFailType::Frontend)) => {
                    // Nothing to do
                }
                IceFailType::Llvm(Some(LlvmFailType::Opt)) => todo!(),
                IceFailType::Llvm(Some(LlvmFailType::UnrecognizedFileFormat)) => {
                    // Objdump?
                    todo!()
                }
                IceFailType::Llvm(Some(LlvmFailType::UnrecognizedOpcode)) => {
                    // Extract asm
                    todo!();
                }
                IceFailType::Gcc(Some(GccFailType::UnrecognizedOpcode)) => {
                    // Extract asm
                    todo!();
                }
                IceFailType::Gcc(Some(GccFailType::InternalCompilerError)) => {
                    // Nothing to do
                }
                IceFailType::Gcc(Some(GccFailType::Lto1Error)) => todo!(),
                IceFailType::Gcc(Some(GccFailType::UnrecognizedInsn)) => todo!(),
                IceFailType::Llvm(Some(LlvmFailType::ReservedRequiredRegister)) => todo!(),
                IceFailType::Llvm(None) => unreachable!(),
                IceFailType::Gcc(None) => unreachable!(),
            }

            generate_bug_report(sh, &fail_info)?;
        }
        FailInfo::Execution(fail_info) => {
            let ignorable_warnings = ignorable_warnings();

            let reproduce_sh = match &fail_info.runner {
                Runner::Native => format!(
                    "#!/bin/bash\n{:?} {} -fsigned-char -fno-strict-aliasing -fwrapv -Wall {} red.c -o red.out\nred.out",
                    fail_info.compiler,
                    reduced_opts[0],
                    ignorable_warnings.join(" ")
                ),
                Runner::Qemu(qemu_config) => {
                    let qemu_cpu = match &qemu_config.cpu_flags {
                        compiler_fuzz::RunnerArguments::Fixed(flags) => flags,
                        compiler_fuzz::RunnerArguments::Generated(generation_script) => &cmd!(
                            sh,
                            "{generation_script} --elf-file-path red.out --print-qemu-cpu"
                        )
                        .read()?,
                    };

		    let qemu = if qemu_cpu.starts_with("rv32") {
			&qemu_config.rv32path
		    } else {
			&qemu_config.rv64path
		    };

                    format!(
			"#!/bin/bash\n{:?} {} -fsigned-char -fno-strict-aliasing -fwrapv -Wall {} red.c -Wall -o red.out\nQEMU_CPU={qemu_cpu} {} red.out",
			fail_info.compiler, reduced_opts[0], ignorable_warnings.join(" "), qemu.to_str().unwrap()
		    )
                }
            };

            sh.write_file(reduction_dir.join("reproduce.sh"), reproduce_sh)?;
            cmd!(sh, "chmod +x reproduce.sh").run()?;
        }
        FailInfo::Runtime(_) => todo!(),
    }

    Ok(())
}

fn categorize_runtime_fail(
    sh: &Shell,
    reduction_dir: &Path,
    _fail_info: &RuntimeFailInfo,
) -> anyhow::Result<RuntimeFailType> {
    assert!(reduction_dir.join("fast_exec_stdout.txt").exists());
    assert!(reduction_dir.join("slow_exec_stdout.txt").exists());

    let fast_stdout = sh.read_file(reduction_dir.join("fast_exec_stdout.txt"))?;
    let slow_stdout = sh.read_file(reduction_dir.join("slow_exec_stdout.txt"))?;

    if fast_stdout != slow_stdout {
        Ok(RuntimeFailType::Mismatch)
    } else {
        panic!("Could not categorize failure!\nfast_stdout:\n{fast_stdout}\nslow_stdout:\n{slow_stdout}");
    }
}

fn categorize_exec_fail(
    sh: &Shell,
    reduction_dir: &Path,
    fail_info: &ExecFailInfo,
) -> anyhow::Result<ExecFailType> {
    assert!(reduction_dir.join("exec_stderr.txt").exists());
    assert!(reduction_dir.join("exec_stdout.txt").exists());
    assert!(reduction_dir.join("exec_signal.txt").exists());

    match fail_info.fail_type {
        ExecFailType::Qemu(_) => {
            let stderr = sh.read_file(reduction_dir.join("exec_stderr.txt"))?;
            let stdout = sh.read_file(reduction_dir.join("exec_stdout.txt"))?;
            let signal = sh.read_file(reduction_dir.join("exec_signal.txt"))?;

            if signal == "4" {
                Ok(ExecFailType::Qemu(Some(QemuFailType::IllegalInsn)))
            } else if signal == "11" {
                Ok(ExecFailType::Qemu(Some(QemuFailType::Segfault)))
            } else if stderr.contains("qemu-riscv") {
                Ok(ExecFailType::Qemu(Some(QemuFailType::ErrorMsg)))
            } else {
                panic!("Could not categorize failure!\nstderr:\n{stderr}\nstdout:\n{stdout}");
            }
        }
        ExecFailType::Native(_) => todo!(),
    }
}

fn categorize_ice_fail(
    sh: &Shell,
    reduction_dir: &Path,
    fail_info: &IceFailInfo,
) -> anyhow::Result<IceFailType> {
    assert!(reduction_dir.join("stderr.txt").exists());

    match fail_info.fail_type {
        IceFailType::Gcc(_) => {
            let stderr = sh.read_file(reduction_dir.join("stderr.txt"))?;

            if stderr.contains("unrecognizable insn") {
                Ok(IceFailType::Gcc(Some(GccFailType::UnrecognizedInsn)))
            } else if stderr.contains("internal compiler error") {
                Ok(IceFailType::Gcc(Some(GccFailType::InternalCompilerError)))
            } else if stderr.contains("unrecognized opcode") {
                Ok(IceFailType::Gcc(Some(GccFailType::UnrecognizedOpcode)))
            } else if stderr.contains("lto1: error:") {
                Ok(IceFailType::Gcc(Some(GccFailType::Lto1Error)))
            } else {
                panic!("Could not categorize failure!\nstderr:\n{stderr}");
            }
        }
        IceFailType::Llvm(_) => {
            let stderr = sh.read_file(reduction_dir.join("stderr.txt"))?;

            if stderr.contains("fatal error: error in backend:") {
                Ok(IceFailType::Llvm(Some(LlvmFailType::Llc)))
            } else if stderr.contains("clang: error: invalid arch name 'rv")
                && stderr.contains("extensions are incompatible")
            {
                Ok(IceFailType::Llvm(Some(LlvmFailType::Frontend)))
            } else if stderr.contains("unrecognized opcode") {
                Ok(IceFailType::Llvm(Some(LlvmFailType::UnrecognizedOpcode)))
            } else if stderr.contains("PLEASE submit a bug report") {
                Ok(IceFailType::Llvm(Some(LlvmFailType::Opt)))
            } else if stderr.contains("file format not recognized") {
                Ok(IceFailType::Llvm(Some(
                    LlvmFailType::UnrecognizedFileFormat,
                )))
            } else {
                panic!("Could not categorize failure!\nstderr:\n{stderr}");
            }
        }
    }
}

fn generate_bug_report(sh: &Shell, fail_info: &IceFailInfo) -> anyhow::Result<()> {
    let compiler = &fail_info.compilers;
    assert_eq!(
        compiler.len(),
        1,
        "Cannot currently support multiple files!"
    );
    let compiler = &compiler[0];
    let testcase_path = &fail_info.testcases[0];

    let flags = sh.read_file("reduced_compiler_opts.txt")?;
    let testcase = sh.read_file(testcase_path)?;

    match fail_info.fail_type {
        IceFailType::Llvm(Some(LlvmFailType::Llc)) => {
            let llvmir = sh.read_file("reduced.ll")?;

            let bug_report = format!("C Testcase:\n```c\n{}\n```\n\nCommand/backtrace:\n```\n{} {}\n\nReduced LLVM IR:\n```llvm ir\n{}\n```\n\nCommand/backtrace:\n```\n{} {} reduced.ll\n```\n\nFound via fuzzer.\n",testcase, compiler.to_str().unwrap(), flags, llvmir, "./llc","Placeholder");

            println!("{bug_report}");

            sh.write_file("bug_report.txt", bug_report)?;

            Ok(())
        }
        IceFailType::Gcc(Some(GccFailType::InternalCompilerError))
        | IceFailType::Gcc(Some(GccFailType::UnrecognizedInsn))
        | IceFailType::Gcc(Some(GccFailType::Lto1Error)) => {
            let compiler = compiler.to_str().unwrap();

            let bug_report = format!(
		    "Testcase:\n{}\n\nCommand/backtrace:\n{} {} {} -c -S -o /dev/null\n{}\n\nFound via fuzzer.\n",
		    testcase, compiler, flags, testcase_path.to_str().unwrap(), "Placeholder"
		);

            println!("{bug_report}");

            sh.write_file("bug_report.txt", bug_report)?;

            Ok(())
        }
        IceFailType::Gcc(Some(GccFailType::UnrecognizedOpcode)) => todo!(),
        IceFailType::Llvm(_) | IceFailType::Gcc(None) => todo!(),
    }
}

fn extract_llvm_ir_llc(
    sh: &Shell,
    reduction_dir: &Path,
    fail_info: &IceFailInfo,
) -> anyhow::Result<()> {
    let compiler = &fail_info.compilers;

    assert_eq!(
        compiler.len(),
        1,
        "Cannot extract/minimize IR from multiple testcases!"
    );
    let compiler = &compiler[0];
    let testcase = &fail_info.testcases[0];

    let compiler_flags_str = sh.read_file(reduction_dir.join("reduced_compiler_opts.txt"))?;
    let compiler_flags = compiler_flags_str.split_whitespace();

    cmd!(
        sh,
        "{compiler} {compiler_flags...} {testcase} -emit-llvm -c -o red.bc"
    )
    .run()?;

    cmd!(
        sh,
        "/scratch/tc-testing/tc-compiler-fuzz-trunk/build-gcv/build-llvm-linux/bin/llvm-dis red.bc"
    )
    .run()?;

    let reduce_sh = "#!/bin/bash\n/scratch/tc-testing/tc-compiler-fuzz-trunk/build-gcv/build-llvm-linux/bin/llc $1 2>&1 | grep -e \"LLVM ERROR\" -e \"Cannot select\"".to_string();
    sh.write_file(reduction_dir.join("min_ir.sh"), reduce_sh)?;

    cmd!(sh, "chmod +x min_ir.sh").run()?;

    cmd!(
        sh,
        "/scratch/tc-testing/tc-compiler-fuzz-trunk/build-gcv/build-llvm-linux/bin/llvm-reduce --test min_ir.sh red.ll"
    )
    .run()?;

    Ok(())
}

fn split_flags(
    sh: &Shell,
    reduction_dir: &Path,
    input_file: PathBuf,
    output_file: PathBuf,
    additional_flags: Option<Vec<String>>,
) -> anyhow::Result<Vec<String>> {
    let compiler_flags_str = sh.read_file(reduction_dir.join(input_file))?;
    let compiler_flags_str = compiler_flags_str
        .replace("-w ", "")
        .replace("-fpermissive ", "");

    let compiler_flags_str = if let Some(additional_flags) = additional_flags {
        compiler_flags_str + " " + &additional_flags.join(" ")
    } else {
        compiler_flags_str
    };

    let compiler_flags = compiler_flags_str
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    // Split up compiler opts to make it easy for creduce to minimize
    let split_flags = compiler_flags_str
        .chars()
        .map(|c| c.to_string())
        .map(|c| if c == " " { "\n".to_string() } else { c })
        .collect::<Vec<_>>()
        .join("\n");

    sh.write_file(reduction_dir.join(output_file), split_flags)?;

    Ok(compiler_flags)
}

fn preprocess(
    sh: &Shell,
    reduction_dir: &Path,
    fail_info: &FailInfo,
) -> anyhow::Result<Vec<PathBuf>> {
    match fail_info {
        FailInfo::Ice(fail_info) => {
            if fail_info.compilers.len() == 1 {
                // Single file

                let generator_flags =
                    get_generator_flags(&fail_info.generator, &fail_info.architecture)?;

                let compiler = &fail_info.compilers;

                match fail_info.generator {
                    FuzzGenerator::Csmith(_) => {
                        let compiler_flags = split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from("compiler_opts.txt"),
                            PathBuf::from("reducible_compiler_opts.txt"),
                            None,
                        )?;

                        assert_eq!(
                            compiler.len(),
                            1,
                            "Csmith should only have one file/compiler!"
                        );
                        let compiler = &compiler[0];

                        let mut run_command: std::process::Command = cmd!(
				sh,
				"{compiler} {generator_flags...} {compiler_flags...} csmith_testcase.c -E -o preprocessed.c"
			)
                        .into();

                        let command_output = run_command.output()?;
                        if !command_output.status.success() {
                            // Initial compile failed, just copy it over, header and all
                            cmd!(sh, "cp csmith_testcase.c preprocessed.c").run()?;
                        }
                        Ok(vec![PathBuf::from("preprocessed.c")])
                    }
                    FuzzGenerator::Fixed(_) => {
                        let _ = split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from("compiler_opts.txt"),
                            PathBuf::from("reducible_compiler_opts.txt"),
                            None,
                        )?;

                        cmd!(sh, "cp fixed_testcase.c preprocessed.c").run()?;
                        Ok(vec![PathBuf::from("preprocessed.c")])
                    }
                    FuzzGenerator::Yarpgen(_) => {
                        let _ = split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from("compiler_opts.txt"),
                            PathBuf::from("reducible_compiler_opts.txt"),
                            Some(generator_flags),
                        )?;

                        Ok(fail_info.testcases.clone())
                    }
                    FuzzGenerator::Rustsmith(_) => todo!(),
                }
            } else {
                // Multiple
                assert!(fail_info.compilers.len() > 1);

                let _ = (0..fail_info.compilers.len())
                    .map(|i| {
                        split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from(format!("compiler_opts_{i}.txt")),
                            PathBuf::from(format!("reducible_compiler_opts_{i}.txt")),
                            None,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();

                match fail_info.generator {
                    FuzzGenerator::Csmith(_) => {
                        panic!("Csmith should only have one file/compiler!");
                    }
                    FuzzGenerator::Fixed(_) => {
                        todo!();
                    }
                    FuzzGenerator::Yarpgen(_) => {
                        // Copy them over so the original files aren't clobbered
                        // when resuming a partial reduction.
                        cmd!(sh, "cp driver.c preprocessed_driver.c").run()?;
                        cmd!(sh, "cp func.c preprocessed_func.c").run()?;

                        Ok(vec![
                            PathBuf::from("preprocessed_driver.c"),
                            PathBuf::from("preprocessed_func.c"),
                        ])
                    }
                    FuzzGenerator::Rustsmith(_) => todo!(),
                }
            }
        }
        FailInfo::Execution(fail_info) => {
            if fail_info.compiler.len() == 1 {
                // Single file
                let compiler_flags = split_flags(
                    sh,
                    reduction_dir,
                    PathBuf::from("compiler_opts.txt"),
                    PathBuf::from("reducible_compiler_opts.txt"),
                    None,
                )?;

                let generator_flags =
                    get_generator_flags(&fail_info.generator, &fail_info.architecture)?;

                let compiler = &fail_info.compiler;

                // TODO: If fails, just copy it over
                match fail_info.generator {
                    FuzzGenerator::Csmith(_) => {
                        assert_eq!(
                            compiler.len(),
                            1,
                            "Csmith should only have one file/compiler!"
                        );
                        let compiler = &compiler[0];

                        let mut run_command: std::process::Command = cmd!(
				sh,
				"{compiler} {generator_flags...} {compiler_flags...} csmith_testcase.c -E -o raw_preprocessed.c"
			    )
                        .into();

                        let command_output = run_command.output()?;
                        if !command_output.status.success() {
                            // Initial compile failed, just copy it over, header and all
                            cmd!(sh, "cp csmith_testcase.c preprocessed.c").run()?;
                        } else {
                            let bash = r#"cat raw_preprocessed.c | tac | sed '/__attribute__ ((__malloc__ (/,/extern/d' | tac > temp.c && mv temp.c preprocessed.c"#;
                            cmd!(sh, "bash -c {bash}").run()?;
                        }

                        Ok(vec![PathBuf::from("preprocessed.c")])
                    }
                    FuzzGenerator::Fixed(_) => {
                        cmd!(sh, "cp fixed_testcase.c preprocessed.c").run()?;
                        Ok(vec![PathBuf::from("preprocessed.c")])
                    }
                    FuzzGenerator::Yarpgen(_) => {
                        // Try without preprocessing
                        Ok(fail_info.testcase.clone())
                    }
                    FuzzGenerator::Rustsmith(_) => todo!(),
                }
            } else {
                // Multiple
                assert!(fail_info.compiler.len() > 1);

                let _ = (0..fail_info.compiler.len())
                    .map(|i| {
                        split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from(format!("compiler_opts_{i}.txt")),
                            PathBuf::from(format!("reducible_compiler_opts_{i}.txt")),
                            None,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();

                match fail_info.generator {
                    FuzzGenerator::Csmith(_) => {
                        panic!("Csmith should only have one file/compiler!");
                    }
                    FuzzGenerator::Fixed(_) => {
                        todo!();
                    }
                    FuzzGenerator::Yarpgen(_) => {
                        Ok(vec![PathBuf::from("driver.c"), PathBuf::from("func.c")])
                    }
                    FuzzGenerator::Rustsmith(_) => todo!(),
                }
            }
        }
        FailInfo::Runtime(fail_info) => {
            if fail_info.fast_compiler.len() == 1 && fail_info.slow_compiler.len() == 1 {
                // Single file
                let slow_generator_flags =
                    get_generator_flags(&fail_info.generator, &fail_info.slow_architecture)?;

                assert!(fail_info.slow_compiler.len() == 1);
                let compiler = &fail_info.slow_compiler[0];

                let _ = split_flags(
                    sh,
                    reduction_dir,
                    PathBuf::from("fast_compiler_opts.txt"),
                    PathBuf::from("fast_reducible_compiler_opts.txt"),
                    None,
                )?;

                let compiler_flags = split_flags(
                    sh,
                    reduction_dir,
                    PathBuf::from("slow_compiler_opts.txt"),
                    PathBuf::from("slow_reducible_compiler_opts.txt"),
                    None,
                )?;

                // TODO: If fails, just copy it over
                match fail_info.generator {
                    FuzzGenerator::Csmith(_) => {
                        let mut run_command: std::process::Command = cmd!(
				    sh,
				    "{compiler} {slow_generator_flags...} {compiler_flags...} csmith_testcase.c -E -o raw_preprocessed.c"
				)
			    .into();

                        let command_output = run_command.output()?;
                        if !command_output.status.success() {
                            // Initial compile failed, just copy it over, header and all
                            cmd!(sh, "cp csmith_testcase.c preprocessed.c").run()?;
                        } else {
                            let bash = r#"cat raw_preprocessed.c | tac | sed '/__attribute__ ((__malloc__ (/,/extern/d' | tac > temp.c && mv temp.c preprocessed.c"#;
                            cmd!(sh, "bash -c {bash}").run()?;
                        }
                    }
                    FuzzGenerator::Fixed(_) => {
                        cmd!(sh, "cp fixed_testcase.c raw_preprocessed.c").run()?;
                    }
                    FuzzGenerator::Yarpgen(_) => todo!(),
                    FuzzGenerator::Rustsmith(_) => todo!(),
                }

                let bash = r#"cat raw_preprocessed.c | tac | sed '/__attribute__ ((__malloc__ (/,/extern/d' | tac | sed -E '/typedef.+_Float/d' > temp.c && mv temp.c preprocessed.c"#;

                cmd!(sh, "bash -c {bash}").run()?;

                Ok(vec![PathBuf::from("preprocessed.c")])
            } else {
                // Multiple
                assert!(fail_info.fast_compiler.len() > 1);
                assert!(fail_info.slow_compiler.len() > 1);

                let _ = (0..fail_info.fast_compiler.len())
                    .map(|i| {
                        split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from(format!("fast_compiler_opts_{i}.txt")),
                            PathBuf::from(format!("fast_reducible_compiler_opts_{i}.txt")),
                            None,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();

                let _ = (0..fail_info.fast_compiler.len())
                    .map(|i| {
                        split_flags(
                            sh,
                            reduction_dir,
                            PathBuf::from(format!("slow_compiler_opts_{i}.txt")),
                            PathBuf::from(format!("slow_reducible_compiler_opts_{i}.txt")),
                            None,
                        )
                        .unwrap()
                    })
                    .collect::<Vec<_>>();

                match fail_info.generator {
                    FuzzGenerator::Csmith(_) => {
                        panic!("Csmith should only have one file/compiler!");
                    }
                    FuzzGenerator::Fixed(_) => {
                        todo!();
                    }
                    FuzzGenerator::Yarpgen(_) => {
                        Ok(vec![PathBuf::from("driver.c"), PathBuf::from("func.c")])
                    }
                    FuzzGenerator::Rustsmith(_) => todo!(),
                }
            }
        }
    }
}
