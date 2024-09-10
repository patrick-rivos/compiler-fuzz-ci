use argh::FromArgs;
use env_logger::Env;
use log::info;

use compiler_flags_gen::{arbitrary_flags, Action, Compiler, FlagSet};

#[derive(FromArgs)]
#[argh(description = "Generate random valid compiler flags

Use RUST_LOG=off to turn off logging")]
struct FlagGen {
    /// emit basic compiler-specific flags
    #[argh(option)]
    flags: Option<String>,

    /// emit flags valid for gcc
    #[argh(switch, short = 'g')]
    gcc: bool,

    /// emit flags valid for llvm
    #[argh(switch, short = 'l')]
    llvm: bool,

    /// emit flags valid for compilation
    #[argh(switch, short = 'c')]
    compile: bool,

    /// emit flags valid for execution
    #[argh(switch, short = 'e')]
    execute: bool,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: FlagGen = argh::from_env();

    info!("Random Flag generator");
    info!("\tflags:\t{:?}", args.flags);
    info!("\tgcc:\t{}", args.gcc);
    info!("\tllvm:\t{}", args.llvm);

    assert!(
        !(args.gcc && args.llvm),
        "Cannot emit flags for GCC and LLVM at the same time."
    );
    assert!(
        (args.gcc || args.llvm),
        "Must specify a compiler (GCC or LLVM)."
    );

    let compiler: Compiler = if args.gcc {
        Compiler::Gcc
    } else {
        Compiler::Llvm
    };

    assert!(
        (args.compile || args.execute),
        "Must specify an action (Compile or Execute)."
    );

    let action: Action = if args.execute {
        Action::Execute
    } else {
        Action::Compile
    };

    let flag_set: FlagSet = match args.flags.as_deref() {
        Some("march") => FlagSet::March,
        Some("march-and-all-flags") => FlagSet::MarchAndAllFlags,
        Some("march-and-basic-flags") => FlagSet::MarchAndBasicFlags,
        Some("all-flags") => FlagSet::AllFlags,
        Some("basic-flags") => FlagSet::BasicFlags,
        Some(_) | None => panic!("Must specify something to emit ('march'/'march-and-all-flags'/'march-and-basic-flags'/'basic-flags'/'all-flags')."),
    };

    let flags = arbitrary_flags(&compiler, &action, &flag_set, false, None);

    println!("{}", flags);
}
