use std::{
    os::unix::process::ExitStatusExt,
    path::{Path, PathBuf},
    process::Output,
    time::Instant,
};

use anyhow::Context;
use xshell::{cmd, Shell};

use crate::{
    Architecture, ExecFailInfo, ExecFailType, FailInfo, FuzzGenerator, Runner, RunnerArguments,
    Stats,
};

pub fn execute_program(
    sh: &Shell,
    exec_ms: &mut u128,
    stats: &mut Stats,
    temp_dir: &Path,
    finds_dir: &Path,
    compiler_paths: &[PathBuf],
    compile_flags: &[String],
    testcase_paths: &[PathBuf],
    architecture: &Architecture,
    generator: &FuzzGenerator,
    program: &Path,
    runner: &Runner,
) -> anyhow::Result<Option<(String, String)>> {
    let exec_timer = Instant::now();

    let input = &match generator {
        FuzzGenerator::Csmith(_) => {
            vec!["1".to_string()]
        }
        FuzzGenerator::Yarpgen(_) => {
            vec![]
        }
        FuzzGenerator::Rustsmith(_) => {
            let run_args = sh.read_file("run_input.txt")?;
            run_args.split_whitespace().map(|s| s.to_string()).collect()
        }
        FuzzGenerator::Fixed(_) => {
            vec![]
        }
    };

    let mut run_command: std::process::Command = match runner {
        Runner::Native => cmd!(sh, "timeout -k 0.1 2 {program} {input...}")
            .quiet()
            .into(),
        Runner::Qemu(qemu_config) => {
            let qemu_cpu = match &qemu_config.cpu_flags {
                RunnerArguments::Fixed(flags) => flags,
                RunnerArguments::Generated(generation_script) => {
                    assert!(cmd!(sh, "/scratch/tc-testing/tc-compiler-fuzz-trunk/build-gcv/bin/riscv64-unknown-linux-gnu-readelf -a {program}").read()?.contains("risc"));
                    &cmd!(
                        sh,
                        "{generation_script} --elf-file-path {program} --print-qemu-cpu"
                    )
                    .read()?
                }
            };

            // TODO: Catch qemu flag generator failures

            let qemu = if qemu_cpu.starts_with("rv32") {
                &qemu_config.rv32path
            } else {
                &qemu_config.rv64path
            };

            cmd!(sh, "timeout -k 0.1 2 {qemu} {program} {input...}")
                .env("QEMU_CPU", qemu_cpu)
                .quiet()
                .into()
        }
    };

    let command_output = run_command.output()?;
    *exec_ms += exec_timer.elapsed().as_millis();

    let triage_info = &ExecTriageInfo {
        command_output: &command_output,
        temp_dir,
        finds_dir,
        compiler_paths,
        testcase_paths,
        flags: compile_flags,
        architecture,
        generator,
        runner,
    };

    triage_execution_command(sh, triage_info, stats)
}

pub struct ExecTriageInfo<'a> {
    command_output: &'a Output, // The exit code/signal/stderr being considered
    temp_dir: &'a Path,         // Where the potential failure is stored
    finds_dir: &'a Path,        // Where to copy this to if it's interesting
    compiler_paths: &'a [PathBuf],
    testcase_paths: &'a [PathBuf],
    flags: &'a [String],
    architecture: &'a Architecture,
    generator: &'a FuzzGenerator,
    runner: &'a Runner,
}

fn triage_execution_command(
    sh: &Shell,
    triage_info: &ExecTriageInfo,
    stats: &mut Stats,
) -> anyhow::Result<Option<(String, String)>> {
    let command_output = triage_info.command_output;
    let temp_dir = triage_info.temp_dir;
    let finds_dir = triage_info.finds_dir;

    let dump_dir = finds_dir.join(temp_dir.file_name().unwrap());

    match command_output.status.code() {
        Some(0) => {
            stats.execute_success += 1;
            Ok(Some((
                String::from_utf8(command_output.stdout.clone()).unwrap(),
                String::from_utf8(command_output.stderr.clone()).unwrap(),
            )))
        }
        Some(124) => {
            stats.execute_timeout += 1;
            Ok(None)
        }
        None => {
            match command_output.status.signal() {
                Some(9) => {
                    // Killed by timeout -k
                    stats.execute_timeout += 1;
                    Ok(None)
                }
                Some(4) => {
                    log_error(sh, triage_info)?;
                    Err(anyhow::anyhow!(
                        "Illegal insn signal for command.\nStdout: {}\nStderr: {}\n Dumped to {:?}",
                        String::from_utf8(command_output.stdout.clone()).unwrap(),
                        String::from_utf8(command_output.stderr.clone()).unwrap(),
                        dump_dir
                    ))
                }
                Some(11) => Ok(None), // TODO: Remove and figure out the native error
                Some(i) => {
                    stats.execute_error += 1;
                    log_error(sh, triage_info)?;
                    Err(anyhow::anyhow!(
                        "Unknown signal `{}' for command.\nStdout: {}\nStderr: {}\n Dumped to {:?}",
                        i,
                        String::from_utf8(command_output.stdout.clone()).unwrap(),
                        String::from_utf8(command_output.stderr.clone()).unwrap(),
                        dump_dir
                    ))
                }
                None => unreachable!("If the exit code is None, the signal must be set!"),
            }
        }
        Some(i) => {
            stats.execute_error += 1;
            log_error(sh, triage_info)?;
            Err(anyhow::anyhow!(
                "Unknown exit code: {i} for command.\nStdout: {}\nStderr: {}\n Dumped to {:?}",
                String::from_utf8(command_output.stdout.clone()).unwrap(),
                String::from_utf8(command_output.stderr.clone()).unwrap(),
                dump_dir
            ))
        }
    }
}

fn log_error(sh: &Shell, triage_info: &ExecTriageInfo) -> anyhow::Result<()> {
    let command_output = triage_info.command_output;
    let temp_dir = triage_info.temp_dir;
    let finds_dir = triage_info.finds_dir;
    let flags = triage_info.flags;
    let compiler_paths = triage_info.compiler_paths;
    let testcase_paths = triage_info.testcase_paths;
    let architecture = triage_info.architecture;
    let generator = triage_info.generator;
    let runner = triage_info.runner;

    cmd!(sh, "cp -r {temp_dir} {finds_dir}")
        .run()
        .context("When attempting to save failure to output directory")?;
    let dump_dir = finds_dir.join(temp_dir.file_name().unwrap());
    if flags.len() == 1 {
        sh.write_file(dump_dir.join("compiler_opts.txt"), flags[0].clone())
            .context("When attempting to save opts to output directory")?;
    } else {
        for (i, flags) in flags.iter().enumerate() {
            sh.write_file(dump_dir.join(format!("compiler_opts_{i}.txt")), flags)
                .context("When attempting to save opts to output directory")?;
        }
    }
    sh.write_file(dump_dir.join("stderr.txt"), &command_output.stderr)
        .context("When attempting to save stderr to output directory")?;
    sh.write_file(dump_dir.join("stdout.txt"), &command_output.stdout)
        .context("When attempting to save stdout to output directory")?;
    sh.write_file(
        dump_dir.join("fail_info.yaml"),
        serde_yaml::to_string(&FailInfo::Execution(ExecFailInfo {
            compiler: compiler_paths.to_vec(),
            architecture: architecture.clone(),
            testcase: testcase_paths.to_vec(),
            generator: generator.clone(),
            runner: runner.clone(),
            fail_type: if matches!(runner, Runner::Qemu(..)) {
                ExecFailType::Qemu(None)
            } else {
                ExecFailType::Native(None)
            },
        }))
        .unwrap(),
    )
    .context("When attempting to save run info to output directory")?;

    Ok(())
}
