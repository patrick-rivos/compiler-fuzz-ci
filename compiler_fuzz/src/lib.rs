use compiler_flags_gen::{Action, Compiler, FlagSet};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod compile;
pub mod execute;
pub mod generate;
pub mod reduction;

pub struct Stats {
    pub compile_success: u128,
    pub compile_timeout: u128,
    pub compile_error: u128,
    pub execute_success: u128,
    pub execute_timeout: u128,
    pub execute_error: u128,
}

/// Config structs/enums

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RunnerArguments {
    Fixed(String),
    Generated(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QemuConfig {
    pub rv32path: PathBuf,
    pub rv64path: PathBuf,
    pub cpu_flags: RunnerArguments,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Runner {
    Native,
    Qemu(QemuConfig),
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlagsGenerator {
    pub compiler: Compiler,
    pub flag_set: FlagSet,
}

#[derive(Deserialize, Debug, Clone)]
pub enum CompilerArguments {
    Fixed(String),
    Generated(FlagsGenerator),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Architecture {
    X86,
    Riscv,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FuzzCompiler {
    pub path: PathBuf,
    pub arguments: CompilerArguments,
    pub runner: Option<Runner>,
    pub architecture: Architecture,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CsmithConfig {
    pub path: PathBuf,
    pub include_dir: PathBuf,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RustsmithConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct YarpgenConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FixedTestcaseConfig {
    pub path: PathBuf,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum FuzzGenerator {
    Csmith(CsmithConfig),
    Yarpgen(YarpgenConfig),
    Rustsmith(RustsmithConfig),
    Fixed(FixedTestcaseConfig),
}

#[derive(Deserialize, Clone)]
pub struct CompileConfig {
    pub action: Action,
    pub compiler: FuzzCompiler,
    pub generator: FuzzGenerator,
}

#[derive(Deserialize)]
pub struct RunConfig {
    pub fast_compiler: FuzzCompiler,
    pub slow_compiler: FuzzCompiler,
    pub generator: FuzzGenerator,
}

#[derive(Deserialize)]
pub enum FuzzConfig {
    Compile(CompileConfig),
    Run(RunConfig),
}

/// Fail info structs/enums

#[derive(Serialize, Deserialize, Debug)]
pub enum LlvmFailType {
    Frontend,
    Llc,
    Opt,
    UnrecognizedOpcode,
    UnrecognizedFileFormat,
    ReservedRequiredRegister,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GccFailType {
    UnrecognizedInsn,
    InternalCompilerError,
    UnrecognizedOpcode,
    Lto1Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum QemuFailType {
    IllegalInsn,
    ErrorMsg,
    Segfault,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NativeFailType {}

#[derive(Serialize, Deserialize, Debug)]
pub enum IceFailType {
    Gcc(Option<GccFailType>),
    Llvm(Option<LlvmFailType>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExecFailType {
    Qemu(Option<QemuFailType>),
    Native(Option<NativeFailType>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RuntimeFailType {
    Mismatch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IceFailInfo {
    pub compilers: Vec<PathBuf>,
    pub architecture: Architecture,
    pub testcases: Vec<PathBuf>,
    pub action: Action,
    pub generator: FuzzGenerator,
    pub fail_type: IceFailType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecFailInfo {
    pub compiler: Vec<PathBuf>,
    pub architecture: Architecture,
    pub testcase: Vec<PathBuf>,
    pub generator: FuzzGenerator,
    pub runner: Runner,
    pub fail_type: ExecFailType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeFailInfo {
    pub fast_compiler: Vec<PathBuf>,
    pub fast_architecture: Architecture,
    pub fast_runner: Runner,
    pub slow_compiler: Vec<PathBuf>,
    pub slow_architecture: Architecture,
    pub slow_runner: Runner,
    pub testcase: Vec<PathBuf>,
    pub generator: FuzzGenerator,
    pub fail_type: Option<RuntimeFailType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FailInfo {
    Ice(IceFailInfo),
    Execution(ExecFailInfo),
    Runtime(RuntimeFailInfo),
}

/// Reduction config structs/enums

pub struct ReductionConfig {}

/// Fns

pub fn ignorable_warnings() -> Vec<String> {
    vec![
        "-Wno-unused-command-line-argument",
        "-Wno-unused-function",
        "-Wno-unused-variable",
        "-Wno-unused-value",
        "-Wno-unused-but-set-variable",
        "-Wno-tautological-constant-out-of-range-compare",
        "-Wno-tautological-pointer-compare",
        "-Wno-tautological-bitwise-compare",
        "-Wno-tautological-compare",
        "-Wno-compare-distinct-pointer-types",
        "-Wno-constant-conversion",
        "-Wno-constant-logical-operand",
        "-Wno-pointer-sign",
        "-Wno-bool-operation",
        "-Wno-parentheses-equality",
        "-Wno-self-assign",
        "-Wno-implicit-const-int-float-conversion",
        // gcc
        "-Wno-unknown-warning-option",
        "-Wno-bool-compare",
        "-Wno-address",
        "-Wno-overflow",
        "-Wno-compare-distinct-pointer-types",
        "-Wno-dangling-pointer",
        // Yarpgen gcc
        "-Wno-int-in-bool-context",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}
