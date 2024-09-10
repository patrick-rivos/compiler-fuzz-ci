use std::{path::PathBuf, process::exit};

use xshell::{cmd, Shell};

use crate::{generate::get_generator_flags, ignorable_warnings, Architecture, FuzzGenerator};

pub fn check_for_ub(
    sh: &Shell,
    generator: &FuzzGenerator,
    testcases: &[PathBuf],
    rv32: bool,
) -> anyhow::Result<()> {
    assert!(!testcases.is_empty());

    let rv32_flags = &if rv32 {
        vec!["-m32", "-malign-double"]
    } else {
        vec![]
    };

    let generator_flags = &get_generator_flags(generator, &Architecture::X86).unwrap();

    let ignorable_warnings = &ignorable_warnings();

    if testcases.len() == 1 {
        let testcase = &testcases[0];

        let stderr = cmd!(sh, "timeout -k 1 4 clang -fsanitize=undefined {testcase} -Wall -Wzero-length-array {ignorable_warnings...} -o clang-ubsan.out -fsigned-char -fno-strict-aliasing -fwrapv {rv32_flags...} {generator_flags...}").quiet().read_stderr()?;

        if stderr.contains("warning:") {
            println!("Unexpected warning! {stderr}");
            exit(1);
        }

        let stderr = cmd!(sh, "timeout -k 1 4 ./clang-ubsan.out")
            .quiet()
            .read_stderr()?;

        if stderr.contains("Error") {
            println!("ubsan found error: {stderr}");
            exit(1);
        }

        cmd!(sh, "timeout -k 1 4 gcc -fsanitize=address {testcase} -w -o gcc-asan.out -fsigned-char -fno-strict-aliasing -fwrapv {ignorable_warnings...} {rv32_flags...} {generator_flags...}").quiet().read()?;

        let stderr = cmd!(sh, "timeout -k 1 4 ./gcc-asan.out")
            .quiet()
            .read_stderr()?;

        if stderr.contains("Error") {
            println!("asan found error: {stderr}");
            exit(1);
        }
    } else {
        // multiple
        // Compile each of the specified files
        let mut object_output_files = vec![];
        assert!(rv32_flags.len() == 0); // TODO
        for testcase in testcases {
            let mut object_output_file = testcase.clone().to_path_buf();
            object_output_file.set_extension("o");

            let mut compile_command: std::process::Command = cmd!(
			    sh,
			    "timeout -k 0.1 5 clang {generator_flags...} -fsanitize=undefined -fsigned-char -fno-strict-aliasing -fwrapv  -Wall -Wzero-length-array {ignorable_warnings...} {testcase} -c -o {object_output_file}"
			)
		.quiet()
		.into();

            object_output_files.append(&mut vec![object_output_file]);

            let command_output = compile_command.output()?;

            if !command_output.status.success() {
                println!(
                    "Intermediate command failed! `{:?}'\nstderr: {}",
                    compile_command,
                    String::from_utf8(command_output.stderr).unwrap()
                );
                exit(1)
            }

            let stderr = String::from_utf8(command_output.stderr)?;
            if stderr.contains("warning:") {
                println!("Unexpected warning with intermediate command! {stderr}");
                exit(1);
            }
        }

        // Link together all the files
        assert!(object_output_files.len() == testcases.len());

        cmd!(
		sh,
		"timeout -k 0.1 5 clang {generator_flags...} -fsanitize=undefined -fsigned-char -fno-strict-aliasing -fwrapv -Wall -Wzero-length-array {ignorable_warnings...} {object_output_files...} -o clang-ubsan.o"
	    )
	    .quiet()
	    .read()?;

        let stderr = cmd!(sh, "timeout -k 1 4 ./clang-ubsan.out")
            .quiet()
            .read_stderr()?;

        if stderr.contains("warning:") {
            println!("Unexpected warning with intermediate command! {stderr}");
            exit(1);
        }

        if stderr.contains("Error") {
            println!("ubsan found error: {stderr}");
            exit(1);
        }
    }

    Ok(())
}

/// Compile code and assert that it succeeds
pub fn compile_clean_code(
    sh: &Shell,
    compilers: &Vec<PathBuf>,
    testcases: &Vec<PathBuf>,
    compiler_flags: &Vec<Vec<String>>,
    output_file: PathBuf,
    ignorable_warnings: &Vec<String>,
) -> anyhow::Result<()> {
    let command_output = if compilers.len() == 1 {
        assert_eq!(testcases.len(), 1);
        assert_eq!(compiler_flags.len(), 1);

        let compiler = &compilers[0];
        let testcase = &testcases[0];
        let compiler_flags = &compiler_flags[0];

        let mut compile_command: std::process::Command = cmd!(
		    sh,
		    "timeout -k 1 4 {compiler} {compiler_flags...} {testcase} -fsigned-char -fno-strict-aliasing -fwrapv -Wall {ignorable_warnings...} -o {output_file}"
		)
		.into();

        println!("{compile_command:?}");

        compile_command.output()?
    } else {
        // multiple files
        assert!(compilers.len() > 1);
        assert!(testcases.len() > 1);
        assert!(
		    compiler_flags.len() == testcases.len() + 1,
		    "{} != {} Must be exactly n+1 sets of flags specified. The last set is used when linking all object files together.", compiler_flags.len(), testcases.len() + 1
		);
        assert!(
			compilers.len() == testcases.len() + 1,
			"Must be exactly n+1 compilers specified. The last compiler is used when linking all object files together."
		    );

        let mut object_output_files = vec![];
        // Compile each of the specified files
        for i in 0..testcases.len() {
            let compiler_path: &str = compilers[i].to_str().unwrap();
            let testcase_path = &testcases[i];
            let flags = &compiler_flags[i];
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
        assert!(object_output_files.len() == testcases.len());

        let flags = &compiler_flags[compiler_flags.len() - 1];
        let compiler_path: &str = compilers[compilers.len() - 1].to_str().unwrap();

        let lto_flag = if compiler_path.contains("clang")
            && compiler_flags
                .iter()
                .any(|v| v.iter().any(|f| f.contains("-flto")))
        {
            vec!["-fuse-ld=lld"]
        } else {
            vec![]
        };

        let mut compile_command: std::process::Command = cmd!(
		sh,
		"timeout -k 0.1 5 {compiler_path} {flags...} {object_output_files...} -Wall {ignorable_warnings...} {lto_flag...} -o {output_file}"
	    )
	    .quiet()
	    .into();

        println!("Command: {compile_command:?}");

        compile_command.output()?
    };

    let stdout = &String::from_utf8(command_output.stdout)?;
    let stderr = String::from_utf8(command_output.stderr)?;

    let output_filename = &output_file
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    sh.write_file(output_filename.clone() + "_stdout.txt", stdout)?;
    sh.write_file(output_filename.clone() + "_stderr.txt", &stderr)?;

    match command_output.status.code().unwrap() {
        0 => {
            // Compilation is expected to pass
            // Make sure there aren't warnings
            if stderr
                .replace("warning: creating DT_TEXTREL in a PIE", "")
                .replace("warning: relocation in read-only section", "")
                .contains("warning:")
            {
                println!("Unexpected {output_filename} compile warning! {stderr}");
                exit(1);
            }
        }
        124 => {
            println!("{output_filename} compilation timeout!");
            exit(1);
        }
        _ => {
            println!(
                "Unrecognized {output_filename} compilation failure: {}",
                stderr
            );
            exit(1);
        }
    };

    Ok(())
}
