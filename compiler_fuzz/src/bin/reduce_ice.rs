use compiler_flags_gen::Action;
use compiler_fuzz::{
    ignorable_warnings, FailInfo, GccFailType, IceFailInfo, IceFailType, LlvmFailType,
};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
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

    let sh = Shell::new()?;
    sh.change_dir(&working_dir);

    match fail_info {
        FailInfo::Ice(fail_info) => produce_fail(&sh, &fail_info, &working_dir)?,
        FailInfo::Execution(_) => unreachable!(),
        FailInfo::Runtime(_) => unreachable!(),
    }

    Ok(())
}

fn produce_fail(sh: &Shell, fail_info: &IceFailInfo, working_dir: &Path) -> anyhow::Result<()> {
    let assemble_link_flags = match fail_info.action {
        Action::Compile => "-S -o /dev/null",
        Action::Assemble => "-c -o /dev/null",
        Action::Link | Action::Execute => "-o testcase.o",
    };
    let assemble_link_flags = &assemble_link_flags.split_whitespace().collect::<Vec<_>>();

    let compilers = &fail_info.compilers;

    let ignorable_warnings = &ignorable_warnings();

    let command_output = if compilers.len() == 1 {
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

        let compiler = &compilers[0];

        let testcase = &fail_info.testcases[0];

        let mut compile_command: std::process::Command = cmd!(
            sh,
            "{compiler} {compiler_flags...} {testcase} -Wall {ignorable_warnings...} {assemble_link_flags...}"
        )
        .into();

        compile_command.output()?
    } else {
        // multiple files
        let testcase_paths = &fail_info.testcases;
        let flags = (0..fail_info.testcases.len() + 1)
            .map(|i| PathBuf::from(format!("reducible_compiler_opts_{i}.txt")))
            .collect::<Vec<_>>();
        assert!(compilers.len() > 1);
        assert!(testcase_paths.len() > 1);
        assert!(
		flags.len() == testcase_paths.len() + 1,
		"{} != {} Must be exactly n+1 sets of flags specified. The last set is used when linking all object files together.", flags.len(), testcase_paths.len() + 1
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

        let mut object_output_files = vec![];
        // Compile each of the specified files
        for i in 0..testcase_paths.len() {
            let compiler_path: &str = compilers[i].to_str().unwrap();
            let testcase_path = &testcase_paths[i];
            let flags = &flags[i];
            let mut object_output_file = testcase_path.clone().to_path_buf();
            object_output_file.set_extension("o");

            let mut compile_command: std::process::Command = cmd!(
		    sh,
		    "timeout -k 0.1 5 {compiler_path} {flags...} {testcase_path} -Wall {ignorable_warnings...} -c -o {object_output_file}"
		)
            .quiet()
            .into();

            object_output_files.append(&mut vec![object_output_file]);

            let command_output = compile_command.output()?;
            let stderr = String::from_utf8(command_output.stderr)?;

            if !command_output.status.success() {
                println!("Intermediate command failed! stderr: {stderr}");
                exit(1)
            }
            if stderr.contains("warning:") {
                println!("Unexpected warning with intermediate command! {stderr}");
                exit(1);
            }
        }

        // Link together all the files
        assert!(object_output_files.len() == testcase_paths.len());

        let flags = &flags[flags.len() - 1];
        let compiler_path: &str = compilers[compilers.len() - 1].to_str().unwrap();

        let mut compile_command: std::process::Command = cmd!(
            sh,
            "timeout -k 0.1 5 {compiler_path} {flags...} {object_output_files...} -Wall {ignorable_warnings...} -o testcase.o"
        )
        .quiet()
        .into();

        compile_command.output()?
    };

    let stdout = String::from_utf8(command_output.stdout)?;
    let stderr = String::from_utf8(command_output.stderr)?;
    sh.write_file("stdout.txt", stdout)?;
    sh.write_file("stderr.txt", &stderr)?;

    if stderr.contains("warning:") {
        println!("Unexpected warning with command! {stderr}");
        exit(1);
    }

    match command_output.status.code().unwrap() {
        0 => {
            println!("This shouldn't pass!");
            exit(1);
        }
        124 => {
            println!("Timeout!");
            exit(1);
        }
        _ => {
            match fail_info.fail_type {
                IceFailType::Llvm(Some(LlvmFailType::Llc)) => {
                    if stderr.contains("fatal error: error in backend:") {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Llvm(Some(LlvmFailType::Frontend)) => {
                    if stderr.contains("clang: error: invalid arch name 'rv")
                        && stderr.contains("extensions are incompatible")
                    {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Llvm(Some(LlvmFailType::UnrecognizedOpcode)) => {
                    if stderr.contains("unrecognized opcode") {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Llvm(Some(LlvmFailType::Opt)) => {
                    if stderr.contains("PLEASE submit a bug report") {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Llvm(Some(LlvmFailType::UnrecognizedFileFormat)) => {
                    if stderr.contains("file format not recognized") {
                        println!("Success: {stderr}");
                        return Ok(());
                    }
                }
                IceFailType::Llvm(Some(LlvmFailType::ReservedRequiredRegister)) => {
                    if stderr.contains("register required, but has been reserved.") {
                        println!("Success: {stderr}");
                        return Ok(());
                    }
                }
                IceFailType::Llvm(None) => {
                    // Has not been categorized, any fail will do.
                    return Ok(());
                }
                IceFailType::Gcc(Some(GccFailType::UnrecognizedInsn)) => {
                    if stderr.contains("unrecognizable insn") {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Gcc(Some(GccFailType::InternalCompilerError)) => {
                    if stderr.contains("internal compiler error") {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Gcc(Some(GccFailType::UnrecognizedOpcode)) => {
                    if stderr.contains("unrecognized opcode") {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Gcc(Some(GccFailType::Lto1Error)) => {
                    if stderr.contains(
                        "lto1: error: '-mdiv' requires '-march' to subsume the 'M' extension",
                    ) {
                        println!("Success");
                        return Ok(());
                    }
                }
                IceFailType::Gcc(None) => {
                    // Has not been categorized, any fail will do.
                    return Ok(());
                }
            }
            println!("Unrecognized failure: {}", stderr);
            exit(1);
        }
    }
}
