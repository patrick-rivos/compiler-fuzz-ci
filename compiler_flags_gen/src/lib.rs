use std::fmt;

use arbitrary::{Arbitrary, Unstructured};
use llvm::BasicLlvmFlags;
use rand::RngCore;
use riscv::{Mabi, March};
use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use crate::gcc::{AllGccFlags, BasicGccFlags};
use crate::llvm::AllLlvmFlags;

pub mod gcc;
pub mod llvm;
pub mod parse_gcc;
pub mod parse_llvm;
pub mod riscv;

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct AllFuzzGcc {
    pub flags: gcc::AllGccFlags,
    pub march: March,
}

impl AllFuzzGcc {
    pub fn sanitize(
        &mut self,
        action: &Action,
        flag_set: &FlagSet,
        rv64_only: bool,
        mabi: Option<Mabi>,
    ) {
        self.flags.sanitize(action);
        self.march
            .sanitize(&Compiler::Gcc, action, flag_set, rv64_only, mabi);
        self.march.m = if self.flags.riscv_toggles.mdiv == ToggleOpt::On {
            !self.march.g
        } else {
            self.march.m
        };

        self.flags.riscv_toggles.mbig_endian = if self.march.implies_vect() {
            GhostOpt::Hidden
        } else {
            self.flags.riscv_toggles.mbig_endian
        }
    }
}

impl fmt::Display for AllFuzzGcc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.march, self.flags)
    }
}

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct BasicFuzzGcc {
    pub flags: gcc::BasicGccFlags,
    pub march: March,
}

impl BasicFuzzGcc {
    pub fn sanitize(
        &mut self,
        action: &Action,
        flag_set: &FlagSet,
        rv64_only: bool,
        mabi: Option<Mabi>,
    ) {
        self.flags.sanitize(action);
        self.march
            .sanitize(&Compiler::Gcc, action, flag_set, rv64_only, mabi);
        self.march.m = if self.flags.riscv_toggles.mdiv == ToggleOpt::On {
            !self.march.g
        } else {
            self.march.m
        };

        // gcc fail: v not supported big endian
        // ld failure: target emulation `elf{32|64}-littleriscv' does not match `elf{32|64}-bigriscv'
        self.flags.riscv_toggles.mbig_endian = if matches!(action, Action::Link)
            || matches!(action, Action::Execute)
            || self.march.implies_vect()
        {
            GhostOpt::Hidden
        } else {
            self.flags.riscv_toggles.mbig_endian
        };

        if self.march.mabi != Mabi::lp64d {
            self.flags.toggles.static_opt = GhostOpt::On;
        }
    }
}

impl fmt::Display for BasicFuzzGcc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.march, self.flags)
    }
}

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct AllFuzzLlvm {
    pub flags: llvm::AllLlvmFlags,
    pub march: March,
}

impl AllFuzzLlvm {
    pub fn sanitize(
        &mut self,
        action: &Action,
        flag_set: &FlagSet,
        rv64_only: bool,
        mabi: Option<Mabi>,
    ) {
        self.flags.sanitize(action);
        self.march
            .sanitize(&Compiler::Llvm, action, flag_set, rv64_only, mabi);

        if self.march.ztso {
            self.flags.fintegrated_as = ToggleOpt::Hidden;
            self.flags.menable_experimental_extensions = GhostOpt::On;
        }
    }
}

impl fmt::Display for AllFuzzLlvm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.march, self.flags)
    }
}

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct BasicFuzzLlvm {
    pub flags: llvm::BasicLlvmFlags,
    pub march: March,
}

impl BasicFuzzLlvm {
    pub fn sanitize(
        &mut self,
        action: &Action,
        flag_set: &FlagSet,
        rv64_only: bool,
        mabi: Option<Mabi>,
    ) {
        self.flags.sanitize(action);
        self.march
            .sanitize(&Compiler::Llvm, action, flag_set, rv64_only, mabi);

        if self.march.mabi != Mabi::lp64d {
            self.flags.toggles.static_opt = GhostOpt::On;
        }

        // TODO: Add mcpu and mtune support
        // // Sanitize mcpu
        // self.flags.riscv_toggles.mcpu = if self.march.mabi.rv32() {
        //     match self.flags.riscv_toggles.mcpu {
        //         Some(llvm::CpuOpt::veyron_v1)
        //         | Some(llvm::CpuOpt::sifive_p450)
        //         | Some(llvm::CpuOpt::sifive_p470)
        //         | Some(llvm::CpuOpt::sifive_p670)
        //         | Some(llvm::CpuOpt::sifive_u54)
        //         | Some(llvm::CpuOpt::generic_rv64)
        //         | Some(llvm::CpuOpt::spacemit_x60)
        //         | Some(llvm::CpuOpt::syntacore_scr3_rv64)
        //         | Some(llvm::CpuOpt::syntacore_scr4_rv64)
        //         | Some(llvm::CpuOpt::syntacore_scr5_rv64)
        //         | Some(llvm::CpuOpt::xiangshan_nanhu)
        //         | Some(llvm::CpuOpt::sifive_s51)
        //         | Some(llvm::CpuOpt::sifive_s54)
        //         | Some(llvm::CpuOpt::sifive_s21)
        //         | Some(llvm::CpuOpt::sifive_x280)
        //         | Some(llvm::CpuOpt::sifive_s76)
        //         | Some(llvm::CpuOpt::rocket_rv64)
        //         | Some(llvm::CpuOpt::sifive_u74) => None,
        //         Some(_) | None => self.flags.riscv_toggles.mcpu,
        //     }
        // } else {
        //     match self.flags.riscv_toggles.mcpu {
        //         Some(llvm::CpuOpt::sifive_e24)
        //         | Some(llvm::CpuOpt::sifive_e21)
        //         | Some(llvm::CpuOpt::sifive_e20)
        //         | Some(llvm::CpuOpt::syntacore_scr3_rv32)
        //         | Some(llvm::CpuOpt::syntacore_scr4_rv32)
        //         | Some(llvm::CpuOpt::syntacore_scr5_rv32)
        //         | Some(llvm::CpuOpt::syntacore_scr1_max)
        //         | Some(llvm::CpuOpt::syntacore_scr1_base)
        //         | Some(llvm::CpuOpt::generic_rv32)
        //         | Some(llvm::CpuOpt::sifive_e31)
        //         | Some(llvm::CpuOpt::sifive_e34)
        //         | Some(llvm::CpuOpt::sifive_e76)
        //         | Some(llvm::CpuOpt::rocket_rv32) => None,
        //         Some(_) | None => self.flags.riscv_toggles.mcpu,
        //     }
        // };

        // // Sanitize mtune
        // self.flags.riscv_toggles.mtune = if self.march.mabi.rv32() {
        //     match self.flags.riscv_toggles.mtune {
        //         Some(llvm::TuneOpt::generic_rv64)
        //         | Some(llvm::TuneOpt::spacemit_x60)
        //         | Some(llvm::TuneOpt::sifive_x280)
        //         | Some(llvm::TuneOpt::sifive_p670)
        //         | Some(llvm::TuneOpt::sifive_p470)
        //         | Some(llvm::TuneOpt::syntacore_scr3_rv64)
        //         | Some(llvm::TuneOpt::syntacore_scr4_rv64)
        //         | Some(llvm::TuneOpt::syntacore_scr5_rv64)
        //         | Some(llvm::TuneOpt::sifive_s51)
        //         | Some(llvm::TuneOpt::veyron_v1)
        //         | Some(llvm::TuneOpt::sifive_s54)
        //         | Some(llvm::TuneOpt::sifive_u74)
        //         | Some(llvm::TuneOpt::sifive_u54)
        //         | Some(llvm::TuneOpt::sifive_s76)
        //         | Some(llvm::TuneOpt::rocket_rv64)
        //         | Some(llvm::TuneOpt::sifive_p450)
        //         | Some(llvm::TuneOpt::sifive_s21)
        //         | Some(llvm::TuneOpt::xiangshan_nanhu) => None,
        //         Some(_) | None => self.flags.riscv_toggles.mtune,
        //     }
        // } else {
        //     match self.flags.riscv_toggles.mtune {
        //         Some(llvm::TuneOpt::generic_rv32)
        //         | Some(llvm::TuneOpt::rocket_rv32)
        //         | Some(llvm::TuneOpt::sifive_e20)
        //         | Some(llvm::TuneOpt::sifive_e21)
        //         | Some(llvm::TuneOpt::sifive_e24)
        //         | Some(llvm::TuneOpt::sifive_e31)
        //         | Some(llvm::TuneOpt::sifive_e34)
        //         | Some(llvm::TuneOpt::sifive_e76)
        //         | Some(llvm::TuneOpt::syntacore_scr1_base)
        //         | Some(llvm::TuneOpt::syntacore_scr1_max)
        //         | Some(llvm::TuneOpt::syntacore_scr3_rv32)
        //         | Some(llvm::TuneOpt::syntacore_scr4_rv32)
        //         | Some(llvm::TuneOpt::syntacore_scr5_rv32) => None,
        //         Some(_) | None => self.flags.riscv_toggles.mtune,
        //     }
        // };

        if !self.march.v {
            self.flags.riscv_toggles.mrvv_vector_bits = None
        }
    }
}

impl fmt::Display for BasicFuzzLlvm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.march, self.flags)
    }
}

#[derive(Arbitrary, Debug, Clone, PartialEq, Copy)]
pub enum ToggleOpt {
    Hidden,
    Off,
    On,
}

#[derive(Arbitrary, Debug, Clone, PartialEq, Copy)]
pub enum GhostOpt {
    Hidden,
    On,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum Action {
    Compile,
    Assemble,
    Link,
    Execute,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum Compiler {
    Gcc,
    Llvm,
    Rustc,
}

#[derive(PartialEq, Deserialize, Debug, Clone)]
pub enum FlagSet {
    MarchAndAllFlags,
    MarchAndBasicFlags,
    March,
    AllFlags,
    BasicFlags,
}

pub fn arbitrary_flags_compatible(
    compiler: &Compiler,
    action: &Action,
    flag_set: &FlagSet,
    rv64_only: bool,
    count: usize,
) -> Vec<String> {
    if count == 1 {
        // No need to worry about ABIs
        return vec![arbitrary_flags(compiler, action, flag_set, false, None)];
    }

    // TODO: Generate compatible but different compiler flags
    //     // Choose ABI
    //     let mut random_bytes = [0u8; 128];
    //     rand::thread_rng().fill_bytes(&mut random_bytes);
    //     let mut unstructured_data = Unstructured::new(random_bytes.as_slice());
    //
    //     let mut mabi = Mabi::arbitrary(&mut unstructured_data).unwrap();
    //     if rv64_only {
    //         mabi = match mabi {
    //             Mabi::ilp32 => Mabi::lp64,
    //             Mabi::ilp32d => Mabi::lp64d,
    //             Mabi::ilp32e => Mabi::lp64e,
    //             Mabi::ilp32f => Mabi::lp64f,
    //             _ => mabi,
    //         }
    //     };

    //     // Ensure specified flags always survive santization
    //     if *action == Action::Link || *action == Action::Execute {
    //         mabi = match mabi {
    //             Mabi::ilp32d | Mabi::lp64d | Mabi::lp64 | Mabi::ilp32 => mabi,
    //             Mabi::ilp32f | Mabi::lp64f | Mabi::ilp32e | Mabi::lp64e => Mabi::lp64d,
    //         }
    //     };
    //     if matches!(compiler, Compiler::Llvm) {
    //         // https://github.com/llvm/llvm-project/issues/100822
    //         if matches!(mabi, Mabi::lp64e) || matches!(mabi, Mabi::ilp32e) {
    //             mabi = Mabi::lp64d;
    //         }
    //     };
    //     // v eventually implies d
    //     if matches!(mabi, Mabi::ilp32e) {
    //         mabi = Mabi::lp64d;
    //     }

    //     // Generate a set of abi-compatible flags
    //     let temp: Vec<_> = (0..count)
    //         .map(|_| arbitrary_flags(compiler, action, flag_set, rv64_only, Some(mabi)))
    //         .collect();

    let flags = arbitrary_flags(compiler, action, flag_set, rv64_only, None);

    let temp: Vec<_> = (0..count).map(|_| flags.clone()).collect();

    assert!(temp.len() == count);

    temp
}

pub fn arbitrary_flags(
    compiler: &Compiler,
    action: &Action,
    flag_set: &FlagSet,
    rv64_only: bool,
    mabi: Option<Mabi>,
) -> String {
    let mut random_bytes = [0u8; 4096];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    let mut unstructured_data = Unstructured::new(random_bytes.as_slice());

    match flag_set {
        FlagSet::MarchAndAllFlags => match compiler {
            Compiler::Gcc => {
                let mut flags = AllFuzzGcc::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action, flag_set, rv64_only, mabi);
                flags.to_string()
            }
            Compiler::Llvm | Compiler::Rustc => {
                let mut flags = AllFuzzLlvm::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action, flag_set, rv64_only, mabi);
                flags.to_string()
            }
        },
        FlagSet::MarchAndBasicFlags => match compiler {
            Compiler::Gcc => {
                let mut flags = BasicFuzzGcc::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action, flag_set, rv64_only, mabi);
                flags.to_string()
            }
            Compiler::Llvm | Compiler::Rustc => {
                let mut flags = BasicFuzzLlvm::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action, flag_set, rv64_only, mabi);
                flags.to_string()
            }
        },
        FlagSet::AllFlags => match compiler {
            Compiler::Gcc => {
                let mut flags = AllGccFlags::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action);
                flags.to_string()
            }
            Compiler::Llvm | Compiler::Rustc => {
                let mut flags = AllLlvmFlags::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action);
                flags.to_string()
            }
        },
        FlagSet::BasicFlags => match compiler {
            Compiler::Gcc => {
                let mut flags = BasicGccFlags::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action);
                flags.to_string()
            }
            Compiler::Llvm | Compiler::Rustc => {
                let mut flags = BasicLlvmFlags::arbitrary(&mut unstructured_data).unwrap();
                flags.sanitize(action);
                flags.to_string()
            }
        },
        FlagSet::March => {
            let mut flags = riscv::March::arbitrary(&mut unstructured_data).unwrap();
            flags.sanitize(compiler, action, flag_set, rv64_only, mabi);
            flags.to_string()
        }
    }
}
