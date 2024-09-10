use std::{
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::Instant,
};

use anyhow::Context;
use compiler_flags_gen::{arbitrary_flags_compatible, Action, Compiler};
use xshell::{cmd, Shell};

use crate::{
    Architecture, CompilerArguments, FailInfo, FlagsGenerator, FuzzCompiler, FuzzGenerator,
    IceFailInfo, IceFailType, Stats,
};

pub fn get_compile_flags(
    compiler: &FuzzCompiler,
    generator: &FuzzGenerator,
    action: &Action,
    base_flags: Vec<&str>,
    count: usize,
) -> Vec<Vec<String>> {
    let rv64_only = match generator {
        FuzzGenerator::Csmith(_) => false,
        FuzzGenerator::Yarpgen(_) => true,
        FuzzGenerator::Rustsmith(_) => false,
        FuzzGenerator::Fixed(_) => false,
    };

    let compiler_flags = match &compiler.arguments {
        CompilerArguments::Generated(FlagsGenerator { compiler, flag_set }) => {
            arbitrary_flags_compatible(compiler, action, flag_set, rv64_only, count)
        }
        CompilerArguments::Fixed(flags) => (0..count).map(|_| flags.clone()).collect(),
    };
    assert_eq!(compiler_flags.len(), count);

    compiler_flags
        .iter()
        .map(|f| {
            let mut compiler_flags = base_flags.clone();
            let mut other_flags = f.split_whitespace().collect::<Vec<_>>();
            compiler_flags.append(&mut other_flags);
            compiler_flags.iter().map(|x| x.to_string()).collect()
        })
        .collect::<Vec<Vec<String>>>()
}

pub fn run_compiler(
    sh: &Shell,
    compiler_ms: &mut u128,
    stats: &mut Stats,
    temp_dir: &Path,
    finds_dir: &Path,
    generator: &FuzzGenerator,
    action: &Action,
    compilers: &[&FuzzCompiler],
    all_flags: &Vec<Vec<&str>>,
    generator_flags: &Vec<&str>,
    testcase_paths: &[PathBuf],
    output_file: &str,
) -> anyhow::Result<bool> {
    let compiler_timer = Instant::now();

    assert!(!testcase_paths.is_empty());

    let result = if testcase_paths.len() == 1 {
        assert!(
            all_flags.len() == 1,
            "Must be exactly one set of flags specified {all_flags:?}"
        );
        assert!(compilers.len() == 1, "Must specify exactly one compiler");

        let compiler_path: &str = compilers[0].path.to_str().unwrap();
        let testcase_path = testcase_paths[0].clone();
        let flags = &all_flags[0];
        let mut compile_command: std::process::Command = cmd!(
		    sh,
		    "timeout -k 0.1 5 {compiler_path} {flags...} {testcase_path} {generator_flags...} -o {output_file}"
		)
		.quiet().into();

        let command_output = compile_command.output()?;
        let triage_info = IceTriageInfo {
            command_output: &command_output,
            temp_dir,
            finds_dir,
            compiler: if compiler_path.contains("gcc") {
                Compiler::Gcc
            } else if compiler_path.contains("clang") {
                Compiler::Llvm
            } else if compiler_path.contains("rustc") {
                Compiler::Rustc
            } else {
                todo!();
            },
            compiler_paths: &[compilers[0].path.clone()],
            testcase_paths: &[testcase_path],
            flags: &vec![flags.iter().map(|s| s.to_string()).collect()],
            architecture: &compilers[0].architecture,
            action: action.clone(),
            generator,
        };

        triage_compile_command(sh, compile_command, &triage_info, stats)?
    } else {
        assert!(testcase_paths.len() > 1);
        assert!(
		    all_flags.len() == testcase_paths.len() + 1,
		"{} != {} Must be exactly n+1 sets of flags specified. The last set is used when linking all object files together.", all_flags.len(), testcase_paths.len() + 1
	    );
        assert!(
	      compilers.len() == testcase_paths.len() + 1,
	      "Must be exactly n+1 compilers specified. The last compiler is used when linking all object files together."
	    );

        let mut object_output_files = vec![];
        // Compile each of the specified files
        for i in 0..testcase_paths.len() {
            let compiler_path: &str = compilers[i].path.to_str().unwrap();
            let testcase_path = &testcase_paths[i];
            let flags = &all_flags[i];
            let mut object_output_file = testcase_path.clone().to_path_buf();
            object_output_file.set_extension("o");

            let mut compile_command: std::process::Command = cmd!(
		    sh,
		    "timeout -k 0.1 5 {compiler_path} {flags...} {testcase_path} {generator_flags...} -c -o {object_output_file}"
		)
		.quiet().into();

            object_output_files.append(&mut vec![object_output_file]);

            let command_output = compile_command.output()?;
            let triage_info = IceTriageInfo {
                command_output: &command_output,
                temp_dir,
                finds_dir,
                compiler: if compiler_path.contains("gcc") {
                    Compiler::Gcc
                } else if compiler_path.contains("clang") {
                    Compiler::Llvm
                } else {
                    todo!()
                },
                compiler_paths: &[compilers[i].path.clone()],
                testcase_paths: &[testcase_path.to_path_buf()],
                flags: &vec![flags.iter().map(|s| s.to_string()).collect()],
                architecture: &compilers[0].architecture,
                action: action.clone(),
                generator,
            };

            let timeout = triage_compile_command(sh, compile_command, &triage_info, stats)?;

            if timeout {
                // Timeout
                *compiler_ms += compiler_timer.elapsed().as_millis();
                return Ok(true);
            }
        }

        // Link together all the files
        assert!(object_output_files.len() == testcase_paths.len());

        let flags = &all_flags[all_flags.len() - 1];
        let compiler_path: &str = compilers[compilers.len() - 1].path.to_str().unwrap();

        let lto_flag = if compiler_path.contains("clang")
            && all_flags
                .iter()
                .any(|v| v.iter().any(|f| f.contains("-flto")))
        {
            vec!["-fuse-ld=lld"]
        } else {
            vec![]
        };

        let mut compile_command: std::process::Command = cmd!(
		    sh,
		    "timeout -k 0.1 5 {compiler_path} {flags...} {object_output_files...} {generator_flags...} {lto_flag...} -o {output_file}"
		)
	    .quiet()
	    .into();

        let command_output = compile_command.output()?;
        let compiler_paths = compilers.iter().map(|c| c.path.clone()).collect::<Vec<_>>();
        let testcase_paths = testcase_paths.to_vec();
        let triage_info = IceTriageInfo {
            command_output: &command_output,
            temp_dir,
            finds_dir,
            compiler: if compiler_path.contains("gcc") {
                Compiler::Gcc
            } else if compiler_path.contains("clang") {
                Compiler::Llvm
            } else {
                todo!()
            },
            compiler_paths: &compiler_paths,
            testcase_paths: &testcase_paths,
            flags: &vec![flags.iter().map(|s| s.to_string()).collect()],
            architecture: &compilers[0].architecture,
            action: action.clone(),
            generator,
        };

        triage_compile_command(sh, compile_command, &triage_info, stats)?
    };
    *compiler_ms += compiler_timer.elapsed().as_millis();
    Ok(result)
}

pub struct IceTriageInfo<'a> {
    command_output: &'a Output, // The exit code/signal/stderr being considered
    temp_dir: &'a Path,         // Where the potential failure is stored
    finds_dir: &'a Path,        // Where to copy this to if it's interesting
    compiler: Compiler,         // The type of compiler that produced command_output
    compiler_paths: &'a [PathBuf],
    testcase_paths: &'a [PathBuf],
    flags: &'a Vec<Vec<String>>,
    architecture: &'a Architecture,
    action: Action,
    generator: &'a FuzzGenerator,
}

fn triage_compile_command(
    sh: &Shell,
    command: Command, // For nice error messages on unknown exit code/signal
    triage_info: &IceTriageInfo,
    stats: &mut Stats,
) -> anyhow::Result<bool> {
    let command_output = triage_info.command_output;
    let temp_dir = triage_info.temp_dir;
    let finds_dir = triage_info.finds_dir;

    let dump_dir = finds_dir.join(temp_dir.file_name().unwrap());

    match command_output.status.code() {
        Some(0) => {
            stats.compile_success += 1;
            Ok(false)
        }
        Some(124) => {
            stats.compile_timeout += 1;
            Ok(true)
        }
        None => {
            match command_output.status.signal() {
                Some(9) => {
                    // Killed by timeout -k
                    stats.compile_timeout += 1;
                    Ok(true)
                }
                Some(i) => {
                    panic!("Stopped by unknown signal {}", i)
                }
                None => unreachable!("If the exit code is None, the signal must be set!"),
            }
        }
        Some(1) => {
            let stderr = String::from_utf8(command_output.stderr.clone()).unwrap();

            if stderr.contains("relocation") {
                // println!("Ignoring relocation truncated, logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if stderr.contains("VPlan cost model and legacy cost model disagreed") {
                println!("Ignoring vplan, logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if stderr
                .contains("Simple vector VT not representable by simple integer vector VT!")
            {
                // println!("Ignoring extending load, logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if matches!(triage_info.compiler, Compiler::Rustc)
                && stderr.contains("error[E0261]: use of undeclared lifetime name")
            {
                println!("Ignoring E0261 (undeclared lifetime name), logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if matches!(triage_info.compiler, Compiler::Rustc)
                && stderr.contains("error[E0506]: cannot assign to")
            {
                println!("Ignoring E0506 (assign to borrowed val), logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if matches!(triage_info.compiler, Compiler::Rustc)
                && stderr.contains("error: lifetime may not live long enough")
            {
                println!("Ignoring error: lifetime may not live long enough, logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if matches!(triage_info.compiler, Compiler::Rustc)
                && stderr.contains("error[E0597]:")
            {
                println!(
                    "Ignoring error[E0597] (val does not live long enough), logging as timeout"
                );
                stats.compile_timeout += 1;
                Ok(true)
            } else if matches!(triage_info.compiler, Compiler::Rustc)
                && stderr.contains("error[E0382]: use of moved value:")
            {
                println!("Ignoring error[E0382] (use of moved value), logging as timeout");
                stats.compile_timeout += 1;
                Ok(true)
            } else if matches!(triage_info.compiler, Compiler::Rustc)
                && stderr.contains("error[E0503]: cannot use")
            {
                println!(
                    "Ignoring error[E0503] (use of mutably borrowed value), logging as timeout"
                );
                stats.compile_timeout += 1;
                Ok(true)
            } else {
                stats.compile_error += 1;
                // Save testcase
                log_error(sh, triage_info)?;
                panic!(
		    "Unknown exit code: 1 for command `{}'.\nStdout: {}\nStderr: {}\n Dumped to {:?}",
		    format!("{:?}", &command).replace("\"", ""), // Remove unneeded quotes. May break commands so be ready to print the compile command directly.
		    String::from_utf8(command_output.stdout.clone()).unwrap(),
		    String::from_utf8(command_output.stderr.clone()).unwrap(),
		    dump_dir
		)
            }
        }
        Some(i) => {
            stats.compile_error += 1;
            // Save testcase
            log_error(sh, triage_info)?;
            panic!(
                "Unknown exit code: {i} for command `{}'.\nStdout: {}\nStderr: {}\n Dumped to {:?}",
                format!("{:?}", &command).replace("\"", ""), // Remove unneeded quotes. May break commands so be ready to print the compile command directly.
                String::from_utf8(command_output.stdout.clone()).unwrap(),
                String::from_utf8(command_output.stderr.clone()).unwrap(),
                dump_dir
            )
        }
    }
}

fn log_error(sh: &Shell, triage_info: &IceTriageInfo) -> anyhow::Result<()> {
    let command_output = triage_info.command_output;
    let temp_dir = triage_info.temp_dir;
    let finds_dir = triage_info.finds_dir;
    let flags = triage_info.flags;
    let compiler_paths = triage_info.compiler_paths;
    let testcase_paths = triage_info.testcase_paths;
    let architecture = triage_info.architecture;
    let action = &triage_info.action;
    let generator = triage_info.generator;
    let compiler = &triage_info.compiler;

    cmd!(sh, "cp -r {temp_dir} {finds_dir}")
        .run()
        .context("When attempting to save failure to output directory")?;
    let dump_dir = finds_dir.join(temp_dir.file_name().unwrap());
    if flags.len() == 1 {
        sh.write_file(
            dump_dir.join("compiler_opts.txt"),
            flags[0].clone().join(" "),
        )
        .context("When attempting to save opts to output directory")?;
    } else {
        for (i, flags) in flags.iter().enumerate() {
            sh.write_file(
                dump_dir.join(format!("compiler_opts_{i}.txt")),
                flags.join(" "),
            )
            .context("When attempting to save opts to output directory")?;
        }
    }
    sh.write_file(dump_dir.join("stderr.txt"), &command_output.stderr)
        .context("When attempting to save stderr to output directory")?;
    sh.write_file(dump_dir.join("stdout.txt"), &command_output.stdout)
        .context("When attempting to save stdout to output directory")?;
    sh.write_file(
        dump_dir.join("fail_info.yaml"),
        serde_yaml::to_string(&FailInfo::Ice(IceFailInfo {
            compilers: compiler_paths.to_vec(),
            architecture: architecture.clone(),
            testcases: testcase_paths.to_vec(),
            action: action.clone(),
            generator: generator.clone(),
            fail_type: match compiler {
                Compiler::Gcc => IceFailType::Gcc(None),
                Compiler::Llvm => IceFailType::Llvm(None),
                Compiler::Rustc => IceFailType::Llvm(None),
            },
        }))
        .unwrap(),
    )
    .context("When attempting to save run info to output directory")?;

    Ok(())
}
