use std::path::PathBuf;

use anyhow::Context;
use xshell::{cmd, Shell};

use crate::{Architecture, FuzzGenerator};

pub fn run_generator(sh: &Shell, generator: &FuzzGenerator) -> anyhow::Result<Vec<PathBuf>> {
    let generator_path: &PathBuf = match &generator {
        FuzzGenerator::Csmith(csmith_config) => &csmith_config.path,
        FuzzGenerator::Yarpgen(yarpgen_config) => &yarpgen_config.path,
        FuzzGenerator::Rustsmith(rustsmith_config) => &rustsmith_config.path,
        FuzzGenerator::Fixed(fixed_config) => &fixed_config.path,
    };

    match &generator {
        FuzzGenerator::Csmith(_) => {
            let csmith_testcase = cmd!(sh, "{generator_path}")
                .quiet()
                .read()
                .context("Generator failed to run")?;
            sh.write_file("csmith_testcase.c", csmith_testcase)?;
            Ok(vec!["csmith_testcase.c".into()])
        }
        FuzzGenerator::Yarpgen(_) => {
            let _ = cmd!(sh, "{generator_path} --std=c")
                .quiet()
                .read()
                .context("Generator failed to run")?;
            Ok(vec!["func.c".into(), "driver.c".into()])
        }
        FuzzGenerator::Rustsmith(_) => {
            let _ = cmd!(sh, "{generator_path} -n 1 --directory rustsmith")
                .quiet()
                .read_stderr()
                .context("Generator failed to run")?;
            let testcase = "rustsmith/file0/file0.rs";
            cmd!(sh, "mv {testcase} rustsmith_testcase.rs")
                .quiet()
                .run()?;
            let run_input = "rustsmith/file0/file0.txt";
            cmd!(sh, "mv {run_input} run_input.txt").quiet().run()?;
            Ok(vec!["rustsmith_testcase.rs".into()])
        }
        FuzzGenerator::Fixed(_) => {
            sh.copy_file(generator_path, "fixed_testcase.c")?;
            Ok(vec!["fixed_testcase.c".into()])
        }
    }
}

pub fn get_generator_flags(
    generator: &FuzzGenerator,
    architecture: &Architecture,
) -> anyhow::Result<Vec<String>> {
    match &generator {
        FuzzGenerator::Csmith(csmith_config) => Ok(vec![format!(
            "-I{}",
            csmith_config.include_dir.to_str().unwrap()
        )]),
        FuzzGenerator::Yarpgen(_) => match architecture {
            Architecture::X86 => Ok(vec!["-mcmodel=large".to_string(), "-fno-pic".to_string()]),
            Architecture::Riscv => Ok(vec!["-mcmodel=medany".to_string()]),
        },
        FuzzGenerator::Rustsmith(_) => Ok(vec![]),
        FuzzGenerator::Fixed(_) => Ok(vec![]),
    }
}
