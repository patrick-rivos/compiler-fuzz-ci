use anyhow::Context;
use argh::FromArgs;
use compiler_fuzz::compile::{get_compile_flags, run_compiler};
use compiler_fuzz::execute::execute_program;
use compiler_fuzz::generate::{get_generator_flags, run_generator};
use env_logger::Env;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::Instant;
use xshell::{cmd, Shell};

use compiler_flags_gen::Action;

use compiler_fuzz::{
    CompileConfig, FailInfo, FuzzCompiler, FuzzConfig, FuzzGenerator, RunConfig, RuntimeFailInfo,
    RuntimeFailType, Stats,
};

#[derive(FromArgs)]
#[argh(description = "Fuzz Compilers
Use RUST_LOG=off to turn off logging")]
struct FuzzArgs {
    /// config file describing the fuzz
    #[argh(positional)]
    config_file: PathBuf,

    /// directory to place the interesting cases
    #[argh(positional)]
    finds_dir: PathBuf,

    /// runner id
    #[argh(positional)]
    id: Option<u32>,
}

fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: FuzzArgs = argh::from_env();

    let mut file = File::open(args.config_file)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let config: FuzzConfig = serde_yaml::from_str(&data)?;

    // Create interesting finds directory
    fs::create_dir_all(&args.finds_dir)?;

    let finds_dir = fs::canonicalize(&args.finds_dir)?;

    fuzz(&config, &finds_dir, args.id)?;

    Ok(())
}

fn fuzz(config: &FuzzConfig, finds_dir: &Path, id: Option<u32>) -> anyhow::Result<()> {
    match config {
        FuzzConfig::Compile(compile_config) => ice_fuzzer(compile_config, finds_dir, id),
        FuzzConfig::Run(run_config) => runtime_diff_fuzzer(run_config, finds_dir, id),
    }
}

fn validate_compiler(compiler: &FuzzCompiler, execute: bool) -> anyhow::Result<()> {
    if !execute {
        assert!(
            compiler.runner.is_none(),
            "Compile fuzzers cannot run code (Runner defined)"
        );
    } else {
        assert!(
            compiler.runner.is_some(),
            "Runtime fuzzers must be able run code (Runner defined)"
        );
    }
    assert!(
        compiler.path.exists(),
        "Compiler({:?}) does not exist!",
        compiler.path
    );

    let sh = Shell::new()?;

    let compiler_path: &str = compiler.path.to_str().unwrap();
    let _ = cmd!(sh, "{compiler_path} --version")
        .quiet()
        .read()
        .context(format!(
            "Compiler({:?}) failed health check (--version)",
            compiler.path
        ))?;

    Ok(())
}

fn validate_generator(generator: &FuzzGenerator) -> anyhow::Result<()> {
    let generator_path: &PathBuf = match generator {
        FuzzGenerator::Csmith(csmith_config) => &csmith_config.path,
        FuzzGenerator::Yarpgen(yarpgen_config) => &yarpgen_config.path,
        FuzzGenerator::Fixed(fixed_config) => &fixed_config.path,
        // TODO: Add c2rust support with csmith/yarpgen
        FuzzGenerator::Rustsmith(rustsmith_config) => &rustsmith_config.path,
    };

    assert!(
        generator_path.exists(),
        "Generator({:?}) does not exist!",
        generator_path
    );

    if matches!(generator, FuzzGenerator::Fixed(..)) {
        // Fixed testcases are just files
        return Ok(());
    }

    let sh = Shell::new()?;

    let generator_path_str: &str = generator_path.to_str().unwrap();
    let _ = cmd!(sh, "{generator_path_str} --help")
        .quiet()
        .read()
        .context(format!(
            "Generator({:?}) failed health check (--help)",
            generator_path
        ))?;

    match generator {
        FuzzGenerator::Csmith(csmith_config) => assert!(
            csmith_config.include_dir.exists(),
            "Generator include dir({:?}) does not exist!",
            csmith_config.include_dir
        ),
        FuzzGenerator::Yarpgen(_yarpgen_config) => {}
        FuzzGenerator::Fixed(_fixed_config) => {}
        FuzzGenerator::Rustsmith(_rustsmith_config) => {}
    };

    Ok(())
}

fn ice_fuzzer(config: &CompileConfig, finds_dir: &Path, id: Option<u32>) -> anyhow::Result<()> {
    validate_compiler(&config.compiler, false)?;
    validate_generator(&config.generator)?;

    let id_str = id.map(|x| x.to_string()).unwrap_or("".to_string());

    // Fuzz
    let sh = Shell::new()?;

    // Create temp directory
    let dir = sh.create_temp_dir().unwrap();
    let temp_dir = dir.path();
    sh.change_dir(temp_dir);

    let assemble_link_flags = match config.action {
        Action::Compile => vec!["-S"],
        Action::Assemble => vec!["-c"],
        Action::Link | Action::Execute => vec![],
    };

    let mut iter = 0;
    let program_timer = Instant::now();
    let mut generator_ms = 0;
    let mut compiler_ms = 0;
    let mut stats = Stats {
        compile_success: 0,
        compile_timeout: 0,
        compile_error: 0,
        execute_success: 0,
        execute_timeout: 0,
        execute_error: 0,
    };
    let mut testcase_paths: Vec<PathBuf> = vec![];
    loop {
        // Generate new testcase
        // This takes a while since it could be csmith running
        if match config.generator {
            // Try n isa strings per testcase
            FuzzGenerator::Csmith(_) => iter % 100 == 0,
            // New testcase every time
            FuzzGenerator::Fixed(_) | FuzzGenerator::Yarpgen(_) | FuzzGenerator::Rustsmith(_) => {
                true
            }
        } {
            let generator_timer = Instant::now();
            testcase_paths = run_generator(&sh, &config.generator)?;
            generator_ms += generator_timer.elapsed().as_millis();
        }

        let generator_flags_strings =
            get_generator_flags(&config.generator, &config.compiler.architecture)?;
        let generator_flags: &Vec<&str> =
            &generator_flags_strings.iter().map(|s| s as &str).collect();

        // Generate flags
        let flag_sets = if testcase_paths.len() == 1 {
            1
        } else {
            testcase_paths.len() + 1
        };

        let flags = if config.compiler.path.to_str().unwrap().contains("rustc") {
            // TODO: Use feature bits (+zfa,+zabha,etc.)
            assert_eq!(flag_sets, 1);

            let flags = get_compile_flags(
                &config.compiler,
                &config.generator,
                &config.action,
                vec![],
                flag_sets,
            );

            let mut flags = if !flags[0].is_empty() {
                assert!(flags[0][0].starts_with("-march"));
                let flags = format!("llvm-args={}", flags[0][0].clone());
                vec![
                    "-C".to_string(),
                    "opt-level=1".to_string(),
                        "-C".to_string(),
                        flags,
                    "--target".to_string(),
                    "riscv64gc-unknown-linux-gnu".to_string(),
		    "-C".to_string(),
		    "linker=/scratch/tc-testing/tc-compiler-fuzz-trunk/build-gcv/bin/riscv64-unknown-linux-gnu-gcc".to_string()
                ]
            } else {
                vec!["-C".to_string(), "opt-level=3".to_string()]
            };

            match config.action {
                Action::Compile => flags.append(&mut vec!["--emit".to_string(), "asm".to_string()]),
                Action::Assemble => {
                    flags.append(&mut vec!["--emit".to_string(), "obj".to_string()])
                } // No integrated assember is not an option in rust: https://github.com/rust-lang/rust/pull/70345
                _ => {}
            }

            vec![flags]
        } else {
            let mut base_flags = vec!["-O3", "-w"];
            base_flags.append(&mut assemble_link_flags.clone());
            if matches!(config.action, Action::Assemble)
                && !config.compiler.path.to_str().unwrap().contains("gcc")
            {
                let mut other_flags = vec!["-no-integrated-as"];
                base_flags.append(&mut other_flags)
            }
            get_compile_flags(
                &config.compiler,
                &config.generator,
                &config.action,
                base_flags,
                flag_sets,
            )
        };
        let flags: Vec<Vec<&str>> = flags
            .iter()
            .map(|v| v.iter().map(|x| x as &str).collect())
            .collect();

        let compilers = if testcase_paths.len() == 1 {
            &vec![&config.compiler]
        } else {
            &(0..testcase_paths.len() + 1)
                .map(|_| &config.compiler)
                .collect()
        };

        // Compile testcase with flags
        run_compiler(
            &sh,
            &mut compiler_ms,
            &mut stats,
            temp_dir,
            finds_dir,
            &config.generator,
            &config.action,
            compilers,
            &flags,
            generator_flags,
            &testcase_paths,
            "output.o",
        )?;

        iter += 1;
        if (iter & (iter - 1)) == 0 {
            println!(
                "{:>2} Iteration: {:>4} - Avg iter length: {:>4} ms (Generator {:>4} ms, Compiler {:>4} ms) - Timeout %: {:>3.3}",
		id_str,
		iter,
                program_timer.elapsed().as_millis() / iter,
		generator_ms / iter,
		compiler_ms / iter,
		(stats.compile_timeout as f64 / (stats.compile_timeout + stats.compile_error + stats.compile_success) as f64) * 100.
            );

            // If we're spending more than 5 ms/iter on rust code, something's going on.
            // Investigate the cause of the regression
            if program_timer.elapsed().as_millis() - (5 * iter) > generator_ms + compiler_ms {
                println!(
                    "WARNING: Spending more than 5ms per iter in harness code! (avg {})",
                    (program_timer.elapsed().as_millis() - (generator_ms + compiler_ms)) / iter
                )
            }
        }

        debug_assert_eq!(stats.execute_success, 0);
        debug_assert_eq!(stats.execute_timeout, 0);
        debug_assert_eq!(stats.execute_error, 0);
    }
}

struct RuntimeStats {
    fast_compiler_stats: Stats,
    slow_compiler_stats: Stats,
    mismatch: u128,
}

fn runtime_diff_fuzzer(
    config: &RunConfig,
    finds_dir: &Path,
    id: Option<u32>,
) -> anyhow::Result<()> {
    validate_compiler(&config.fast_compiler, true)?;
    validate_compiler(&config.slow_compiler, true)?;

    let id_str = id.map(|x| x.to_string()).unwrap_or("".to_string());

    // Fuzz
    let sh = Shell::new()?;

    // Create temp directory
    let dir = sh.create_temp_dir().unwrap();
    let temp_dir = dir.path();
    sh.change_dir(temp_dir);

    let mut iter = 0;
    let program_timer = Instant::now();
    let mut generator_ms = 0;
    let mut fast_compiler_ms = 0;
    let mut fast_exec_ms = 0;
    let mut slow_compiler_ms = 0;
    let mut slow_exec_ms = 0;
    let mut stats = RuntimeStats {
        fast_compiler_stats: Stats {
            compile_success: 0,
            compile_timeout: 0,
            compile_error: 0,
            execute_success: 0,
            execute_timeout: 0,
            execute_error: 0,
        },
        slow_compiler_stats: Stats {
            compile_success: 0,
            compile_timeout: 0,
            compile_error: 0,
            execute_success: 0,
            execute_timeout: 0,
            execute_error: 0,
        },
        mismatch: 0,
    };
    let mut testcase_paths: Vec<PathBuf> = vec![];
    loop {
        // Generate new testcase
        // This takes a while since it could be csmith running
        if iter % 10 == 0 {
            // Try 10 isa strings per testcase
            let generator_timer = Instant::now();
            testcase_paths = run_generator(&sh, &config.generator)?;
            generator_ms += generator_timer.elapsed().as_millis();
        }

        let fast_generator_flags_strings =
            get_generator_flags(&config.generator, &config.fast_compiler.architecture)?;
        let fast_generator_flags: &Vec<&str> = &fast_generator_flags_strings
            .iter()
            .map(|s| s as &str)
            .collect();

        let flag_sets = if testcase_paths.len() == 1 {
            1
        } else {
            testcase_paths.len() + 1
        };

        // Adjust fast flags based on rv32/64 of slow runner
        let slow_base_flags = if config
            .slow_compiler
            .path
            .to_str()
            .unwrap()
            .contains("rustc")
        {
            vec![]
        } else {
            vec![
                "-w",
                "-fpermissive",
                "-fno-strict-aliasing",
                "-fwrapv",
                "-fsigned-char",
                "-O3",
            ]
        };

        let slow_runner_flags = get_compile_flags(
            &config.slow_compiler,
            &config.generator,
            &Action::Execute,
            slow_base_flags,
            flag_sets,
        );

        let slow_runner_flags = if config
            .slow_compiler
            .path
            .to_str()
            .unwrap()
            .contains("rustc")
        {
            if !slow_runner_flags[0].is_empty() {
                assert!(slow_runner_flags[0][0].starts_with("-march"));
                let slow_runner_flags = format!("llvm-args={}", slow_runner_flags[0][0].clone());
                vec![vec![
                    "-C".to_string(),
                    "opt-level=1".to_string(),
                        "-C".to_string(),
                        slow_runner_flags,
                    "--target".to_string(),
                    "riscv64gc-unknown-linux-gnu".to_string(),
		    "-C".to_string(),
		    "linker=/scratch/tc-testing/tc-compiler-fuzz-trunk/build-gcv/bin/riscv64-unknown-linux-gnu-gcc".to_string()
                ]]
            } else {
                vec![vec!["-C".to_string(), "opt-level=3".to_string()]]
            }
        } else {
            slow_runner_flags
        };

        let slow_runner_flags: Vec<Vec<&str>> = slow_runner_flags
            .iter()
            .map(|v| v.iter().map(|x| x as &str).collect())
            .collect();

        let mut fast_base_flags = if config
            .fast_compiler
            .path
            .to_str()
            .unwrap()
            .contains("rustc")
        {
            vec![]
        } else {
            vec![
                "-w",
                "-fpermissive",
                "-fno-strict-aliasing",
                "-fwrapv",
                "-fsigned-char",
                "-O1",
            ]
        };
        if slow_runner_flags
            .iter()
            .any(|s| s.iter().any(|s| s.contains("march=rv32")))
        {
            fast_base_flags.append(&mut vec!["-m32", "-malign-double"]);
        }

        // Generate flags
        let fast_runner_flags = get_compile_flags(
            &config.fast_compiler,
            &config.generator,
            &Action::Execute,
            fast_base_flags,
            flag_sets,
        );
        let fast_runner_flags = if config
            .slow_compiler
            .path
            .to_str()
            .unwrap()
            .contains("rustc")
        {
            if !fast_runner_flags[0].is_empty() {
                assert!(fast_runner_flags[0][0].starts_with("-march"));
                let fast_runner_flags = format!("llvm-args={}", fast_runner_flags[0][0].clone());
                vec![vec!["-C".to_string(), "opt-level=3".to_string()]]
            } else {
                vec![vec!["-C".to_string(), "opt-level=3".to_string()]]
            }
        } else {
            fast_runner_flags
        };
        let fast_runner_flags: Vec<Vec<&str>> = fast_runner_flags
            .iter()
            .map(|v| v.iter().map(|x| x as &str).collect())
            .collect();

        let fast_compilers = if testcase_paths.len() == 1 {
            &vec![&config.fast_compiler]
        } else {
            &(0..testcase_paths.len() + 1)
                .map(|_| &config.fast_compiler)
                .collect()
        };

        if run_compiler(
            &sh,
            &mut fast_compiler_ms,
            &mut stats.fast_compiler_stats,
            temp_dir,
            finds_dir,
            &config.generator,
            &Action::Execute,
            fast_compilers,
            &fast_runner_flags,
            fast_generator_flags,
            &testcase_paths,
            "fast_compiler.out",
        )? {
            // timeout
            continue;
        }

        let fast_runner_flags_strs = fast_runner_flags
            .iter()
            .map(|x| x.join(" "))
            .collect::<Vec<String>>();
        let fast_compiler_paths = fast_compilers
            .iter()
            .map(|c| c.path.clone())
            .collect::<Vec<PathBuf>>();

        let fast_stdout = execute_program(
            &sh,
            &mut fast_exec_ms,
            &mut stats.fast_compiler_stats,
            temp_dir,
            finds_dir,
            &fast_compiler_paths,
            &fast_runner_flags_strs,
            &testcase_paths,
            &config.fast_compiler.architecture,
            &config.generator,
            &temp_dir.join("fast_compiler.out"),
            &config.fast_compiler.runner.clone().unwrap(),
        )
        .context("Fast compiler");

        if fast_stdout.is_err() {
            println!("Ignoring x86 exec error, logging as timeout");
            stats.fast_compiler_stats.execute_timeout += 1;
            continue;
        }

        let fast_stdout = fast_stdout.unwrap();

        if fast_stdout.is_none() {
            // timeout
            continue;
        }

        let slow_generator_flags_strings =
            get_generator_flags(&config.generator, &config.slow_compiler.architecture)?;
        let slow_generator_flags: &Vec<&str> = &slow_generator_flags_strings
            .iter()
            .map(|s| s as &str)
            .collect();

        let slow_compilers = if testcase_paths.len() == 1 {
            &vec![&config.slow_compiler]
        } else {
            &(0..testcase_paths.len() + 1)
                .map(|_| &config.slow_compiler)
                .collect()
        };

        if run_compiler(
            &sh,
            &mut slow_compiler_ms,
            &mut stats.slow_compiler_stats,
            temp_dir,
            finds_dir,
            &config.generator,
            &Action::Execute,
            slow_compilers,
            &slow_runner_flags,
            slow_generator_flags,
            &testcase_paths,
            "slow_compiler.out",
        )
        .context("Slow compiler")?
        {
            // timeout
            continue;
        }

        let slow_runner_flags_strs = slow_runner_flags
            .iter()
            .map(|x| x.join(" "))
            .collect::<Vec<String>>();
        let slow_compiler_paths = slow_compilers
            .iter()
            .map(|c| c.path.clone())
            .collect::<Vec<PathBuf>>();

        let slow_stdout = execute_program(
            &sh,
            &mut slow_exec_ms,
            &mut stats.slow_compiler_stats,
            temp_dir,
            finds_dir,
            &slow_compiler_paths,
            &slow_runner_flags_strs,
            &testcase_paths,
            &config.slow_compiler.architecture,
            &config.generator,
            &temp_dir.join("slow_compiler.out"),
            &config.slow_compiler.runner.clone().unwrap(),
        )?;

        if slow_stdout.is_none() {
            // timeout
            continue;
        }

        let (fast_stdout, fast_stderr) = fast_stdout.unwrap();
        let (slow_stdout, slow_stderr) = slow_stdout.unwrap();

        if fast_stdout != slow_stdout {
            stats.mismatch += 1;
            // Save testcase
            println!("Runner id: {id_str}");
            cmd!(sh, "cp -r {temp_dir} {finds_dir}")
                .run()
                .context("When attempting to save failure to output directory")?;
            let dump_dir = finds_dir.join(temp_dir.file_name().unwrap());
            if fast_compilers.len() == 1 {
                sh.write_file(
                    dump_dir.join(format!("fast_compiler_opts.txt")),
                    fast_runner_flags[0].join(" "),
                )
                .context("When attempting to save opts to output directory")?;
                sh.write_file(
                    dump_dir.join(format!("slow_compiler_opts.txt")),
                    slow_runner_flags[0].join(" "),
                )
                .context("When attempting to save opts to output directory")?;
            } else {
                for (i, flags) in fast_runner_flags.iter().enumerate() {
                    sh.write_file(
                        dump_dir.join(format!("fast_compiler_opts_{i}.txt")),
                        flags.join(" "),
                    )
                    .context("When attempting to save opts to output directory")?;
                }
                for (i, flags) in slow_runner_flags.iter().enumerate() {
                    sh.write_file(
                        dump_dir.join(format!("slow_compiler_opts_{i}.txt")),
                        flags.join(" "),
                    )
                    .context("When attempting to save opts to output directory")?;
                }
            }
            sh.write_file(dump_dir.join("fast_stderr.txt"), &fast_stderr)
                .context("When attempting to save fast_stderr to output directory")?;
            sh.write_file(dump_dir.join("fast_stdout.txt"), &fast_stdout)
                .context("When attempting to save fast_stdout to output directory")?;
            sh.write_file(dump_dir.join("slow_stderr.txt"), &slow_stderr)
                .context("When attempting to save slow_stderr to output directory")?;
            sh.write_file(dump_dir.join("slow_stdout.txt"), &slow_stdout)
                .context("When attempting to save slow_stdout to output directory")?;
            sh.write_file(
                dump_dir.join("fail_info.yaml"),
                serde_yaml::to_string(&FailInfo::Runtime(RuntimeFailInfo {
                    fast_compiler: if fast_compilers.len() == 1 {
                        vec![fast_compilers[0].path.clone()]
                    } else {
                        (0..testcase_paths.len() + 1)
                            .map(|_| config.fast_compiler.path.clone())
                            .collect()
                    },
                    fast_architecture: config.fast_compiler.architecture.clone(),
                    fast_runner: config.fast_compiler.runner.clone().unwrap(),
                    slow_compiler: if slow_compilers.len() == 1 {
                        vec![config.slow_compiler.path.clone()]
                    } else {
                        (0..testcase_paths.len() + 1)
                            .map(|_| config.slow_compiler.path.clone())
                            .collect()
                    },
                    slow_architecture: config.slow_compiler.architecture.clone(),
                    slow_runner: config.slow_compiler.runner.clone().unwrap(),
                    testcase: testcase_paths,
                    fail_type: Some(RuntimeFailType::Mismatch),
                    generator: config.generator.clone(),
                }))
                .unwrap(),
            )
            .context("When attempting to save run info to output directory")?;
            panic!(
                "Stdout mismatch {} != {}\n Dumped to {:?}",
                fast_stdout, slow_stdout, dump_dir
            );
        }

        iter += 1;
        if (iter & (iter - 1)) == 0 {
            println!(
                "{:>2} Iteration: {:>4} - Avg iter length: {:>4} ms (Generator {:>4} ms, fast|slow compile {:>4}|{:>4} ms, fast|slow exec {:>4}|{:>4} ms) fast|slow c timeout %: {:>3.3}|{:>3.3} fast|slow exec timeout %: {:>3.3}|{:>3.3}",
		id_str,
		iter,
                program_timer.elapsed().as_millis() / iter,
		generator_ms / iter,
		fast_compiler_ms / iter,
		slow_compiler_ms / iter,
		fast_exec_ms / iter,
		slow_exec_ms / iter,
		((stats.fast_compiler_stats.compile_timeout as f64) / (stats.fast_compiler_stats.compile_timeout + stats.fast_compiler_stats.compile_error + stats.fast_compiler_stats.compile_success) as f64) * 100.,
		((stats.slow_compiler_stats.compile_timeout as f64) / (stats.slow_compiler_stats.compile_timeout + stats.slow_compiler_stats.compile_error + stats.slow_compiler_stats.compile_success) as f64) * 100.,
		((stats.fast_compiler_stats.execute_timeout as f64) / (stats.fast_compiler_stats.execute_timeout + stats.fast_compiler_stats.execute_error + stats.fast_compiler_stats.execute_success) as f64) * 100.,
		((stats.slow_compiler_stats.execute_timeout as f64) / (stats.slow_compiler_stats.execute_timeout + stats.slow_compiler_stats.execute_error + stats.slow_compiler_stats.execute_success) as f64) * 100.,
            );

            // If we're spending more than 5 ms/iter on rust code, something's going on.
            // Investigate the cause of the regression
            if program_timer.elapsed().as_millis() - (5 * iter)
                > generator_ms + fast_compiler_ms + slow_compiler_ms + fast_exec_ms + slow_exec_ms
            {
                println!(
                    "WARNING: Spending more than 5ms per iter in harness code! (avg {})",
                    (program_timer.elapsed().as_millis()
                        - (generator_ms
                            + fast_compiler_ms
                            + slow_compiler_ms
                            + fast_exec_ms
                            + slow_exec_ms))
                        / iter
                )
            }
        }
    }
}
