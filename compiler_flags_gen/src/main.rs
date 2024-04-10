use arbitrary::Arbitrary;
use arbitrary::Unstructured;
use argh::FromArgs;
use compiler_flags_gen::gcc::GccFlags;
use compiler_flags_gen::llvm::LlvmFlags;
use compiler_flags_gen::riscv::March;
use compiler_flags_gen::FuzzGcc;
use compiler_flags_gen::FuzzLlvm;
use env_logger::Env;
use log::info;
use rand::RngCore;

fn arbitrary_flags(march: bool, gcc: bool, llvm: bool) -> String {
    assert!(
        !(gcc && llvm),
        "Cannot emit flags for GCC and LLVM at the same time."
    );

    let mut random_bytes = [0u8; 2048];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    let mut unstructured_data = Unstructured::new(random_bytes.as_slice());

    if march && gcc {
        let mut flags = FuzzGcc::arbitrary(&mut unstructured_data).unwrap();
        flags.sanitize();
        flags.to_string()
    } else if march && llvm {
        let mut flags = FuzzLlvm::arbitrary(&mut unstructured_data).unwrap();
        flags.sanitize();
        flags.to_string()
    } else if march {
        let mut flags = March::arbitrary(&mut unstructured_data).unwrap();
        flags.sanitize();
        flags.to_string()
    } else if gcc {
        let mut flags = GccFlags::arbitrary(&mut unstructured_data).unwrap();
        flags.sanitize();
        flags.to_string()
    } else if llvm {
        let mut flags = LlvmFlags::arbitrary(&mut unstructured_data).unwrap();
        flags.sanitize();
        flags.to_string()
    } else {
        "".to_string()
    }
}

#[derive(FromArgs)]
#[argh(description = "Generate random valid compiler flags

Use RUST_LOG=off to turn off logging")]
struct FlagGen {
    /// emit a march string
    #[argh(switch, short = 'm')]
    march: bool,

    /// emit gcc flags
    #[argh(switch, short = 'g')]
    gcc: bool,

    /// emit llvm flags
    #[argh(switch, short = 'l')]
    llvm: bool,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let args: FlagGen = argh::from_env();

    info!("Random Flag generator");
    info!("\tmarch:\t{}", args.march);
    info!("\tgcc:\t{}", args.gcc);
    info!("\tllvm:\t{}", args.llvm);

    let flags = arbitrary_flags(args.march, args.gcc, args.llvm);

    println!("{}", flags);
}
