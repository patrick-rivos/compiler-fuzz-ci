use std::fmt;

use arbitrary::Arbitrary;
use riscv::March;
use struct_iterable::Iterable;

pub mod gcc;
pub mod llvm;
pub mod parse_gcc;
pub mod parse_llvm;
pub mod riscv;

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct FuzzGcc {
    pub flags: gcc::GccFlags,
    pub march: March,
}

impl FuzzGcc {
    pub fn sanitize(&mut self) {
        self.flags.sanitize();
        self.march.sanitize();
        self.march.m = if self.flags.riscv_toggles.mdiv == ToggleOpt::On {
            !self.march.g
        } else {
            self.march.m
        };

        self.flags.riscv_toggles.mbig_endian = if self.march.v
            || self.march.zvbb
            || self.march.zvbc
            || self.march.zve32f
            || self.march.zve32x
            || self.march.zve64d
            || self.march.zve64f
            || self.march.zve64x
            || self.march.zvfh
            || self.march.zvfhmin
            || self.march.zvkg
            || self.march.zvkn
            || self.march.zvknc
            || self.march.zvkned
            || self.march.zvkng
            || self.march.zvknha
            || self.march.zvknhb
            || self.march.zvks
            || self.march.zvksc
            || self.march.zvksed
            || self.march.zvksg
            || self.march.zvksh
            || self.march.zvkt
        {
            GhostOpt::Hidden
        } else {
            self.flags.riscv_toggles.mbig_endian
        }
    }
}

impl fmt::Display for FuzzGcc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.march, self.flags)
    }
}

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct FuzzLlvm {
    pub flags: llvm::LlvmFlags,
    pub march: March,
}

impl FuzzLlvm {
    pub fn sanitize(&mut self) {
        self.flags.sanitize();
        self.march.sanitize();

        if self.march.ztso0p1 {
            self.flags.fintegrated_as = ToggleOpt::Hidden;
            self.flags.menable_experimental_extensions = GhostOpt::On;
        }

        //clang: error: invalid arch name 'rv64g_xtheadfmv', unsupported non-standard user-level extension 'xtheadfmv'
        self.march.xtheadfmv = false;
        //clang: error: invalid arch name 'rv64g_xtheadint', unsupported non-standard user-level extension 'xtheadint'
        self.march.xtheadint = false;
    }
}

impl fmt::Display for FuzzLlvm {
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
