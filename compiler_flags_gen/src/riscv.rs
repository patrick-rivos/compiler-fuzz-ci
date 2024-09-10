use std::{collections::HashSet, fmt};

use arbitrary::Arbitrary;
use struct_iterable::Iterable;

use crate::{Action, Compiler, FlagSet};

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct March {
    pub mabi: Mabi,
    pub g: bool,
    i: bool,
    e: bool,
    a: bool,
    b: bool,
    c: bool,
    d: bool,
    f: bool,
    h: bool,
    pub m: bool,
    pub v: bool,
    smaia: bool,
    smepmp: bool,
    smstateen: bool,
    ssaia: bool,
    sscofpmf: bool,
    ssstateen: bool,
    sstc: bool,
    svinval: bool,
    svnapot: bool,
    svpbmt: bool,
    xcvalu: bool,
    xcvbi: bool,
    xcvelw: bool,
    xcvmac: bool,
    xcvsimd: bool,
    xsfcease: bool,
    pub xsfvcp: bool,
    xtheadba: bool,
    xtheadbb: bool,
    xtheadbs: bool,
    xtheadcmo: bool,
    xtheadcondmov: bool,
    xtheadfmemidx: bool,
    pub xtheadfmv: bool,
    pub xtheadint: bool,
    xtheadmac: bool,
    xtheadmemidx: bool,
    xtheadmempair: bool,
    xtheadsync: bool,
    xtheadvector: bool,
    xventanacondops: bool,
    za128rs: bool,
    za64rs: bool,
    zaamo: bool,
    zabha: bool,
    zalrsc: bool,
    zawrs: bool,
    zba: bool,
    zbb: bool,
    zbc: bool,
    zbkb: bool,
    zbkc: bool,
    zbkx: bool,
    zbs: bool,
    zca: bool,
    zcb: bool,
    zcd: bool,
    zce: bool,
    zcf: bool,
    zcmp: bool,
    zcmt: bool,
    zdinx: bool,
    zfa: bool,
    zfbfmin: bool,
    zfh: bool,
    zfhmin: bool,
    zfinx: bool,
    zhinx: bool,
    zhinxmin: bool,
    zic64b: bool,
    zicbom: bool,
    zicbop: bool,
    zicboz: bool,
    ziccamoa: bool,
    ziccif: bool,
    zicclsm: bool,
    ziccrse: bool,
    zicntr: bool,
    zicond: bool,
    zicsr: bool,
    zifencei: bool,
    zihintntl: bool,
    zihintpause: bool,
    zihpm: bool,
    zk: bool,
    zkn: bool,
    zknd: bool,
    zkne: bool,
    zknh: bool,
    zkr: bool,
    zks: bool,
    zksed: bool,
    zksh: bool,
    zkt: bool,
    zmmul: bool,
    pub ztso: bool,
    pub zvbb: bool,
    pub zvbc: bool,
    pub zve32f: bool,
    pub zve32x: bool,
    pub zve64d: bool,
    pub zve64f: bool,
    pub zve64x: bool,
    zvfbfmin: bool,
    zvfbfwma: bool,
    pub zvfh: bool,
    pub zvfhmin: bool,
    zvkb: bool,
    pub zvkg: bool,
    pub zvkn: bool,
    pub zvknc: bool,
    pub zvkned: bool,
    pub zvkng: bool,
    pub zvknha: bool,
    pub zvknhb: bool,
    pub zvks: bool,
    pub zvksc: bool,
    pub zvksed: bool,
    pub zvksg: bool,
    pub zvksh: bool,
    pub zvkt: bool,
    shcounterenw: bool,
    shgatpa: bool,
    shtvala: bool,
    shvsatpa: bool,
    shvstvala: bool,
    shvstvecd: bool,
    smcdeleg: bool,
    smcsrind: bool,
    ssccfg: bool,
    ssccptr: bool,
    sscounterenw: bool,
    sscsrind: bool,
    ssstrict: bool,
    sstvala: bool,
    sstvecd: bool,
    ssu64xl: bool,
    svade: bool,
    svadu: bool,
    svbare: bool,
    xcvbitmanip: bool,
    xcvmem: bool,
    xsfvfnrclipxfqf: bool,
    xsfvfwmaccqqq: bool,
    xsfvqmaccdod: bool,
    xsfvqmaccqoq: bool,
    xsifivecdiscarddlone: bool,
    xsifivecflushdlone: bool,
    xtheadvdot: bool,
    xwchc: bool,
    zama16b: bool,
    zcmop: bool,
    zimop: bool,
    zvl: Zvl,
    vendor_extensions: AllowedVendorExtensions,
}

#[derive(Arbitrary, Debug, Clone, Copy)]
enum AllowedVendorExtensions {
    Nil,
    Xthead,
    Xsifive,
    Xventana,
    Xcorev,
    Xw,
}

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Clone, Copy, PartialEq)]
pub enum Mabi {
    ilp32,
    ilp32d,
    ilp32e,
    ilp32f,
    lp64,
    lp64d,
    lp64e,
    lp64f,
}

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Clone, Copy, PartialEq)]
enum Zvl {
    zvl32b,
    zvl64b,
    zvl128b,
    zvl256b,
    zvl512b,
    zvl1024b,
    zvl2048b,
    zvl4096b,
    // zvl8192b, // Current RISC-V GCC does not support VLEN greater than 4096bit for 'V' Extension
    // zvl16384b,
    // zvl32768b,
    // zvl65536b,
    nozvl,
}

impl March {
    pub fn implies_vect(&self) -> bool {
        self.v
            || self.xsfvcp
            || self.zvfh
            || self.zvkb
            || self.zvfhmin
            || self.zvfbfmin
            || self.zvfbfwma
            || self.zvkg
            || self.zvkn
            || self.zvknc
            || self.zvknha
            || self.zvknhb
            || self.zvkng
            || self.zvkt
            || self.zvks
            || self.zvksg
            || self.zvksed
            || self.zvksh
            || self.xtheadvector
            || self.xtheadvdot
    }

    fn disable_gcc_unsupported_exts(&mut self) {
        self.shcounterenw = false;
        self.shgatpa = false;
        self.shtvala = false;
        self.shvsatpa = false;
        self.shvstvala = false;
        self.shvstvecd = false;
        self.smcdeleg = false;
        self.smcsrind = false;
        self.ssccfg = false;
        self.ssccptr = false;
        self.sscounterenw = false;
        self.sscsrind = false;
        self.ssstrict = false;
        self.sstvala = false;
        self.sstvecd = false;
        self.ssu64xl = false;
        self.svade = false;
        self.svadu = false;
        self.svbare = false;
        self.xcvbitmanip = false;
        self.xcvmem = false;
        self.xsfvfnrclipxfqf = false;
        self.xsfvfwmaccqqq = false;
        self.xsfvqmaccdod = false;
        self.xsfvqmaccqoq = false;
        self.xsifivecdiscarddlone = false;
        self.xsifivecflushdlone = false;
        self.xtheadvdot = false;
        self.xwchc = false;
        self.zama16b = false;
        self.zcmop = false;
        self.zimop = false;
    }

    // TODO: Double check
    fn disable_binutils_unsupported_exts(&mut self) {
        self.shcounterenw = false;
        self.shgatpa = false;
        self.shtvala = false;
        self.shvsatpa = false;
        self.shvstvala = false;
        self.shvstvecd = false;
        self.smcdeleg = false;
        self.smcsrind = false;
        self.ssccfg = false;
        self.ssccptr = false;
        self.sscounterenw = false;
        self.sscsrind = false;
        self.ssstrict = false;
        self.sstvala = false;
        self.sstvecd = false;
        self.ssu64xl = false;
        self.svade = false;
        self.svadu = false;
        self.svbare = false;
        self.xcvbitmanip = false;
        self.xcvmem = false;
        self.xsfvfnrclipxfqf = false;
        self.xsfvfwmaccqqq = false;
        self.xsfvqmaccdod = false;
        self.xsfvqmaccqoq = false;
        self.xsifivecdiscarddlone = false;
        self.xsifivecflushdlone = false;
        self.xtheadvdot = false;
        self.xwchc = false;
        self.zama16b = false;
        self.zcmop = false;
        self.zimop = false;

        // https://sourceware.org/bugzilla/show_bug.cgi?id=32036
        self.zcmp = false;

        // Binutils doesn't support rv32 xventanacondops https://sourceware.org/bugzilla/show_bug.cgi?id=32037
        if self.mabi.rv32() {
            self.xventanacondops = false;
        }
    }

    fn disable_qemu_unsupported_exts(&mut self) {
        self.xcvalu = false;
        self.xcvmac = false;

        // TODO: Triage, this segfaults QEMU under certain circumstances
        self.xsfvcp = false;
        self.zvfh = false;
    }

    pub fn sanitize(
        &mut self,
        compiler: &Compiler,
        action: &Action,
        flag_set: &FlagSet,
        rv64_only: bool,
        mabi: Option<Mabi>,
    ) {
        // Turn off vector for better coverage of non-vector toggles.
        // self.v = false;
        // self.zvbb = false;
        // self.zvbc = false;
        // self.zvkned = false;
        // self.zve32f = false;
        // self.zve32x = false;
        // self.zve64d = false;
        // self.zve64f = false;
        // self.zve64x = false;
        // self.zvksc = false;
        // self.zvl = Zvl::nozvl;
        // self.zcd = false;
        // self.zvkb = false;
        // self.zvkg = false;
        // self.zvkn = false;
        // self.zvknc = false;
        // self.zvkned = false;
        // self.zvkng = false;
        // self.zvknha = false;
        // self.zvknhb = false;
        // self.zvks = false;
        // self.zvksc = false;
        // self.zvksed = false;
        // self.zvksg = false;
        // self.zvksh = false;
        // self.zvkt = false;
        // self.zvfh = false;
        // self.zvfhmin = false;
        // self.xtheadvector = false;
        // self.xtheadvdot = false;
        // self.xsfvfnrclipxfqf = false;
        // self.xsfvfwmaccqqq = false;
        // self.xsfvqmaccdod = false;
        // self.xsfvqmaccqoq = false;
        // self.zvfbfmin = false;
        // self.zvfbfwma = false;

        if let Some(mabi) = mabi {
            self.mabi = mabi
        }

        if rv64_only {
            self.mabi = match self.mabi {
                Mabi::ilp32 => Mabi::lp64,
                Mabi::ilp32d => Mabi::lp64d,
                Mabi::ilp32e => Mabi::lp64e,
                Mabi::ilp32f => Mabi::lp64f,
                _ => self.mabi,
            }
        }

        match compiler {
            Compiler::Gcc => {
                self.disable_gcc_unsupported_exts();

                match action {
                    Action::Compile => {}
                    Action::Assemble => {
                        self.zic64b = false;
                        self.ziccif = false;
                        self.ziccamoa = false;
                        self.zicclsm = false;
                        self.ziccrse = false;
                        self.zfbfmin = false;
                        self.za128rs = false;
                        self.za64rs = false;
                        self.zvfbfmin = false;
                        self.zce = false;
                        self.zcmt = false;
                        self.zfbfmin = false;
                        self.zvfbfwma = false;

                        // Must be set with version
                        self.xcvelw = false;
                        self.xcvsimd = false;
                        self.xcvbi = false;
                        self.xsfcease = false;

                        self.disable_binutils_unsupported_exts();
                    }
                    Action::Link => {
                        self.zic64b = false;
                        self.ziccif = false;
                        self.ziccamoa = false;
                        self.zicclsm = false;
                        self.ziccrse = false;
                        self.zfbfmin = false;
                        self.za128rs = false;
                        self.za64rs = false;
                        self.zvfbfmin = false;
                        self.zce = false;
                        self.zcmt = false;
                        self.zfbfmin = false;
                        self.zvfbfwma = false;

                        self.disable_binutils_unsupported_exts();

                        // Must be set with version
                        self.xcvelw = false;
                        self.xcvsimd = false;
                        self.xcvbi = false;
                        self.xsfcease = false;
                    }
                    Action::Execute => {
                        self.zic64b = false;
                        self.ziccif = false;
                        self.ziccamoa = false;
                        self.zicclsm = false;
                        self.ziccrse = false;
                        self.zfbfmin = false;
                        self.za128rs = false;
                        self.za64rs = false;
                        self.zvfbfmin = false;
                        self.zce = false;
                        self.zcmt = false;
                        self.zfbfmin = false;
                        self.zvfbfwma = false;

                        self.disable_binutils_unsupported_exts();

                        // Must be set with version
                        self.xcvelw = false;
                        self.xcvsimd = false;
                        self.xcvbi = false;
                        self.xsfcease = false;

                        self.disable_qemu_unsupported_exts();

                        self.zvl = match self.zvl {
                            Zvl::zvl32b => Zvl::nozvl,
                            Zvl::zvl64b => Zvl::nozvl,
                            Zvl::zvl2048b => Zvl::nozvl,
                            Zvl::zvl4096b => Zvl::nozvl,
                            i => i, // All other zvls are fine
                        }
                    }
                }
            }
            Compiler::Llvm | Compiler::Rustc => {
                match action {
                    Action::Compile => {}
                    Action::Assemble => {
                        self.zic64b = false;
                        self.ziccif = false;
                        self.ziccamoa = false;
                        self.zicclsm = false;
                        self.ziccrse = false;
                        self.zfbfmin = false;
                        self.za128rs = false;
                        self.za64rs = false;
                        self.zvfbfmin = false;
                        self.zce = false;
                        self.zcmt = false;
                        self.zfbfmin = false;
                        self.zvfbfwma = false;

                        self.disable_binutils_unsupported_exts();

                        // Must be set with version
                        self.xcvelw = false;
                        self.xcvsimd = false;
                        self.xcvbi = false;
                        self.xcvmem = false;
                        self.xcvbitmanip = false;
                        self.xtheadvdot = false;
                        self.xsfcease = false;
                        self.xsfvfnrclipxfqf = false;
                        self.xsfvqmaccdod = false;
                        self.xsfvfwmaccqqq = false;
                        self.xsfvqmaccqoq = false;
                        self.xsifivecdiscarddlone = false;
                        self.xsifivecflushdlone = false;
                        self.xwchc = false;

                        // Binutils doesn't support rv32 xventanacondops https://sourceware.org/bugzilla/show_bug.cgi?id=32037
                        if self.mabi.rv32() {
                            self.xventanacondops = false;
                        }
                    }
                    Action::Link => {
                        self.zic64b = false;
                        self.ziccif = false;
                        self.ziccamoa = false;
                        self.zicclsm = false;
                        self.ziccrse = false;
                        self.zfbfmin = false;
                        self.za128rs = false;
                        self.za64rs = false;
                        self.zvfbfmin = false;
                        self.zce = false;
                        self.zcmt = false;
                        self.zfbfmin = false;
                        self.zvfbfwma = false;

                        self.disable_binutils_unsupported_exts();

                        // Must be set with version
                        self.xcvelw = false;
                        self.xcvsimd = false;
                        self.xcvbi = false;
                        self.xcvmem = false;
                        self.xcvbitmanip = false;
                        self.xtheadvdot = false;
                        self.xsfcease = false;
                        self.xsfvfnrclipxfqf = false;
                        self.xsfvqmaccdod = false;
                        self.xsfvfwmaccqqq = false;
                        self.xsfvqmaccqoq = false;
                        self.xsifivecdiscarddlone = false;
                        self.xsifivecflushdlone = false;
                        self.xwchc = false;
                    }
                    Action::Execute => {
                        self.zic64b = false;
                        self.ziccif = false;
                        self.ziccamoa = false;
                        self.zicclsm = false;
                        self.ziccrse = false;
                        self.zfbfmin = false;
                        self.za128rs = false;
                        self.za64rs = false;
                        self.zvfbfmin = false;
                        self.zce = false;
                        self.zcmt = false;
                        self.zfbfmin = false;
                        self.zvfbfwma = false;

                        self.disable_binutils_unsupported_exts();

                        // Must be set with version
                        self.xcvelw = false;
                        self.xcvsimd = false;
                        self.xcvbi = false;
                        self.xcvmem = false;
                        self.xcvbitmanip = false;
                        self.xtheadvdot = false;
                        self.xsfcease = false;
                        self.xsfvfnrclipxfqf = false;
                        self.xsfvqmaccdod = false;
                        self.xsfvfwmaccqqq = false;
                        self.xsfvqmaccqoq = false;
                        self.xsifivecdiscarddlone = false;
                        self.xsifivecflushdlone = false;
                        self.xwchc = false;

                        self.disable_qemu_unsupported_exts();

                        self.zvl = match self.zvl {
                            Zvl::zvl32b => Zvl::nozvl,
                            Zvl::zvl64b => Zvl::nozvl,
                            Zvl::zvl2048b => Zvl::nozvl,
                            Zvl::zvl4096b => Zvl::nozvl,
                            i => i, // All other zvls are fine
                        };
                    }
                }

                // https://github.com/llvm/llvm-project/issues/100822
                if matches!(self.mabi, Mabi::lp64e) || matches!(self.mabi, Mabi::ilp32e) {
                    self.mabi = Mabi::lp64d;
                }

                self.zaamo = self.zaamo || self.zabha;

                // Unsupported
                self.xtheadfmv = false;
                self.xtheadint = false;
                self.xtheadvector = false;

                // https://github.com/llvm/llvm-project/issues/102249
                // 'zvk*' requires 'v' or 'zve*' extension to also be specified
                self.v = self.v
                    || self.zvkb
                    || self.zvkg
                    || self.zvkn
                    || self.zvknc
                    || self.zvkned
                    || self.zvkng
                    || self.zvknha
                    || self.zvknhb
                    || self.zvks
                    || self.zvksc
                    || self.zvksed
                    || self.zvksg
                    || self.zvksh
                    || self.zvkt;
            }
        }

        // https://github.com/llvm/llvm-project/issues/100814
        // v eventually implies d
        if matches!(self.mabi, Mabi::ilp32e) && (self.d || self.v || self.implies_vect()) {
            self.mabi = Mabi::lp64d;
        }

        if *flag_set == FlagSet::March && (*action == Action::Link || *action == Action::Execute) {
            // We won't be able to set -static, so disallow all other -mabis
            // Without this we get a ld error: 'error adding symbols: file in wrong format'
            self.mabi = Mabi::lp64d;
        }

        self.mabi = if *action == Action::Link || *action == Action::Execute {
            match self.mabi {
                Mabi::ilp32d | Mabi::lp64d | Mabi::lp64 | Mabi::ilp32 => self.mabi,
                Mabi::ilp32f | Mabi::lp64f | Mabi::ilp32e | Mabi::lp64e => Mabi::lp64d,
            }
        } else {
            self.mabi
        };

        // G disables all extensions it represents
        self.g = self.i && self.m && self.a && self.f && self.d && self.zifencei && self.zicsr;
        self.i = self.i && (!self.g);
        self.m = self.m && (!self.g);
        self.a = self.a && (!self.g);
        self.f = self.f && (!self.g);
        self.d = self.d && (!self.g);
        self.zifencei = self.zifencei && (!self.g);
        self.zicsr = self.zicsr && (!self.g);

        // E requires ilp32e/lp64e ABI
        self.i = match self.mabi {
            Mabi::ilp32e => self.i,
            _ => !self.g, // Set i if not already set via g
        };

        // Use e if i is not set
        self.e = !self.i && !self.g;

        // zvksc expands to zvks and zvbc
        self.zvks = self.zvks || self.zvksc;
        self.zvbc = self.zvbc || self.zvksc;

        self.h = self.h && self.i;
        self.d = (self.d
            || (match self.mabi {
                Mabi::ilp32 => false,
                Mabi::ilp32d => true,
                Mabi::ilp32e => false,
                Mabi::ilp32f => true,
                Mabi::lp64 => false,
                Mabi::lp64d => true,
                Mabi::lp64e => false,
                Mabi::lp64f => false,
            }))
            && (!self.g);
        self.f = (self.f
            || (match self.mabi {
                Mabi::ilp32 => false,
                Mabi::ilp32d => false,
                Mabi::ilp32e => false,
                Mabi::ilp32f => true,
                Mabi::lp64 => false,
                Mabi::lp64d => false,
                Mabi::lp64e => false,
                Mabi::lp64f => true,
            }))
            && (!self.g);

        self.v = self.v
            || (self.zvbb
                || self.zvbc
                || self.zvkned
                || self.zve32f
                || self.zve32x
                || self.zve64d
                || self.zve64f
                || self.zve64x
                || self.zvl != Zvl::nozvl);

        self.xtheadvector = self.xtheadvector && !self.implies_vect();

        // v implies zve64d which eventually implies f & d.
        // z*inx conflicts with floating point extensions.
        self.zfinx = self.zfinx
            && !self.v
            && !self.zvfh // Implies v
	    && !self.zvfhmin
            && !self.f
            && !self.d
            && !self.h
            && !self.g
            && !self.zfh
            && !self.zfhmin
            && !self.zvfbfwma
            && !self.zvfbfmin
            && !self.zfbfmin
            && !self.zcf
	    && !self.xsfvfwmaccqqq
	    && !self.xsfvfnrclipxfqf
	    && !self.zfa;
        self.zhinxmin = self.zhinxmin && self.zfinx;
        self.zdinx = self.zdinx && self.zfinx;
        self.zhinx = self.zhinx && self.zfinx;

        self.xwchc = self.xwchc
            && !self.d
            && !self.g
            && !self.v
            && !self.zcb
            && !self.c
            && !self.xtheadvdot
            && !self.zce;

        // zcf extension supports in rv32 only
        self.zcf = self.zcf && self.mabi.rv32();

        // zcd conficts with zcmp/zcmt
        self.zcmp = self.zcmp && !self.zcd && !self.c;
        self.zcmt = self.zcmt && !self.zcd && !self.c;
        // zce implies zcmp/zcmt
        self.zce = self.zce && !self.zcd && !self.c;

        match compiler {
            Compiler::Gcc => {
                // v requries m
                self.m = (self.m || self.implies_vect()) && !self.g;

                // https://gcc.gnu.org/bugzilla/show_bug.cgi?id=116131
                self.xtheadmemidx = false;
            }
            Compiler::Llvm | Compiler::Rustc => {
                // 'Xwchc' is only supported for 'rv32'
                self.xwchc = self.xwchc && self.mabi.rv32();
            }
        }

        if let Some(mabi) = mabi {
            assert_eq!(
                self.mabi, mabi,
                "Specified mabi did not survive santitization"
            )
        }
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.iter()
            .map(|(field_name, field_value)| {
                if field_value.is::<bool>() {
                    let value: bool = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    if value {
                        format!("-{}", arg_name)
                    } else {
                        format!("-{}no-{}", &arg_name[0..1], &arg_name[1..])
                    }
                } else {
                    panic!("Unknown datatype for field: {}", field_name)
                }
            })
            .collect::<Vec<_>>()
    }
}

impl Mabi {
    pub fn rv32(&self) -> bool {
        match self {
            Mabi::ilp32 => true,
            Mabi::ilp32d => true,
            Mabi::ilp32e => true,
            Mabi::ilp32f => true,
            Mabi::lp64 => false,
            Mabi::lp64d => false,
            Mabi::lp64e => false,
            Mabi::lp64f => false,
        }
    }
}

impl fmt::Display for March {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut abi = None;

        let flags = self
            .iter()
            .map(|(field_name, field_value)| {
                if field_value.is::<bool>() {
                    let value: bool = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    if value {
                        if arg_name.len() == 1 {
                            arg_name
                        } else {
                            format!("_{arg_name}")
                        }
                    } else {
                        "".to_string()
                    }
                } else if field_value.is::<Zvl>() {
                    let value: Zvl = *field_value.downcast_ref().unwrap();
                    match value {
                        Zvl::zvl32b => "_zvl32b".to_string(),
                        Zvl::zvl64b => "_zvl64b".to_string(),
                        Zvl::zvl128b => "_zvl128b".to_string(),
                        Zvl::zvl256b => "_zvl256b".to_string(),
                        Zvl::zvl512b => "_zvl512b".to_string(),
                        Zvl::zvl1024b => "_zvl1024b".to_string(),
                        Zvl::zvl2048b => "_zvl2048b".to_string(),
                        Zvl::zvl4096b => "_zvl4096b".to_string(),
                        // Zvl::zvl8192b => "_zvl8192b".to_string(),
                        // Zvl::zvl16384b => "_zvl16384b".to_string(),
                        // Zvl::zvl32768b => "_zvl32768b".to_string(),
                        // Zvl::zvl65536b => "_zvl65536b".to_string(),
                        Zvl::nozvl => "".to_string(),
                    }
                } else if field_value.is::<Mabi>() {
                    let value: Mabi = *field_value.downcast_ref().unwrap();
                    abi = Some(value);
                    if value.rv32() {
                        "rv32".to_string()
                    } else {
                        "rv64".to_string()
                    }
                } else if field_value.is::<AllowedVendorExtensions>() {
                    // Do nothing
                    "".to_string()
                } else {
                    panic!("Unknown datatype for field: {}", field_name)
                }
            })
            .collect::<Vec<_>>();

        let abi = match abi {
            Some(Mabi::ilp32) => "ilp32",
            Some(Mabi::ilp32d) => "ilp32d",
            Some(Mabi::ilp32e) => "ilp32e",
            Some(Mabi::ilp32f) => "ilp32f",
            Some(Mabi::lp64) => "lp64",
            Some(Mabi::lp64d) => "lp64d",
            Some(Mabi::lp64e) => "lp64e",
            Some(Mabi::lp64f) => "lp64f",
            None => panic!(),
        };

        write!(f, "-march={} -mabi={}", flags.join(""), abi)
    }
}

fn _convert_extensions_to_struct() {
    let march_help_exts = "i                       2.0, 2.1
    e                       2.0
    m                       2.0
    a                       2.0, 2.1
    f                       2.0, 2.2
    d                       2.0, 2.2
    c                       2.0
    b                       1.0
    v                       1.0
    h                       1.0
    zic64b                  1.0
    zicbom                  1.0
    zicbop                  1.0
    zicboz                  1.0
    ziccamoa                1.0
    ziccif                  1.0
    zicclsm                 1.0
    ziccrse                 1.0
    zicntr                  2.0
    zicond                  1.0
    zicsr                   2.0
    zifencei                2.0
    zihintntl               1.0
    zihintpause             2.0
    zihpm                   2.0
    zmmul                   1.0
    za128rs                 1.0
    za64rs                  1.0
    zaamo                   1.0
    zabha                   1.0
    zalrsc                  1.0
    zawrs                   1.0
    zfa                     1.0
    zfbfmin                 1.0
    zfh                     1.0
    zfhmin                  1.0
    zfinx                   1.0
    zdinx                   1.0
    zca                     1.0
    zcb                     1.0
    zcd                     1.0
    zce                     1.0
    zcf                     1.0
    zcmp                    1.0
    zcmt                    1.0
    zba                     1.0
    zbb                     1.0
    zbc                     1.0
    zbkb                    1.0
    zbkc                    1.0
    zbkx                    1.0
    zbs                     1.0
    zk                      1.0
    zkn                     1.0
    zknd                    1.0
    zkne                    1.0
    zknh                    1.0
    zkr                     1.0
    zks                     1.0
    zksed                   1.0
    zksh                    1.0
    zkt                     1.0
    ztso                    1.0
    zvbb                    1.0
    zvbc                    1.0
    zve32f                  1.0
    zve32x                  1.0
    zve64d                  1.0
    zve64f                  1.0
    zve64x                  1.0
    zvfbfmin                1.0
    zvfbfwma                1.0
    zvfh                    1.0
    zvfhmin                 1.0
    zvkb                    1.0
    zvkg                    1.0
    zvkn                    1.0
    zvknc                   1.0
    zvkned                  1.0
    zvkng                   1.0
    zvknha                  1.0
    zvknhb                  1.0
    zvks                    1.0
    zvksc                   1.0
    zvksed                  1.0
    zvksg                   1.0
    zvksh                   1.0
    zvkt                    1.0
    zvl1024b                1.0
    zvl128b                 1.0
    zvl16384b               1.0
    zvl2048b                1.0
    zvl256b                 1.0
    zvl32768b               1.0
    zvl32b                  1.0
    zvl4096b                1.0
    zvl512b                 1.0
    zvl64b                  1.0
    zvl65536b               1.0
    zvl8192b                1.0
    zhinx                   1.0
    zhinxmin                1.0
    smaia                   1.0
    smepmp                  1.0
    smstateen               1.0
    ssaia                   1.0
    sscofpmf                1.0
    ssstateen               1.0
    sstc                    1.0
    svinval                 1.0
    svnapot                 1.0
    svpbmt                  1.0
    xcvalu                  1.0
    xcvbi                   1.0
    xcvelw                  1.0
    xcvmac                  1.0
    xcvsimd                 1.0
    xsfcease                1.0
    xsfvcp                  1.0
    xtheadba                1.0
    xtheadbb                1.0
    xtheadbs                1.0
    xtheadcmo               1.0
    xtheadcondmov           1.0
    xtheadfmemidx           1.0
    xtheadfmv               1.0
    xtheadint               1.0
    xtheadmac               1.0
    xtheadmemidx            1.0
    xtheadmempair           1.0
    xtheadsync              1.0
    xtheadvector            1.0
    xventanacondops         1.0";

    let llvm_print_supported_exts = "i                    2.1       'I' (Base Integer Instruction Set)
    e                    2.0       Implements RV{32,64}E (provides 16 rather than 32 GPRs)
    m                    2.0       'M' (Integer Multiplication and Division)
    a                    2.1       'A' (Atomic Instructions)
    f                    2.2       'F' (Single-Precision Floating-Point)
    d                    2.2       'D' (Double-Precision Floating-Point)
    c                    2.0       'C' (Compressed Instructions)
    b                    1.0       'B' (the collection of the Zba, Zbb, Zbs extensions)
    v                    1.0       'V' (Vector Extension for Application Processors)
    h                    1.0       'H' (Hypervisor)
    zic64b               1.0       'Zic64b' (Cache Block Size Is 64 Bytes)
    zicbom               1.0       'Zicbom' (Cache-Block Management Instructions)
    zicbop               1.0       'Zicbop' (Cache-Block Prefetch Instructions)
    zicboz               1.0       'Zicboz' (Cache-Block Zero Instructions)
    ziccamoa             1.0       'Ziccamoa' (Main Memory Supports All Atomics in A)
    ziccif               1.0       'Ziccif' (Main Memory Supports Instruction Fetch with Atomicity Requirement)
    zicclsm              1.0       'Zicclsm' (Main Memory Supports Misaligned Loads/Stores)
    ziccrse              1.0       'Ziccrse' (Main Memory Supports Forward Progress on LR/SC Sequences)
    zicntr               2.0       'Zicntr' (Base Counters and Timers)
    zicond               1.0       'Zicond' (Integer Conditional Operations)
    zicsr                2.0       'zicsr' (CSRs)
    zifencei             2.0       'Zifencei' (fence.i)
    zihintntl            1.0       'Zihintntl' (Non-Temporal Locality Hints)
    zihintpause          2.0       'Zihintpause' (Pause Hint)
    zihpm                2.0       'Zihpm' (Hardware Performance Counters)
    zimop                1.0       'Zimop' (May-Be-Operations)
    zmmul                1.0       'Zmmul' (Integer Multiplication)
    za128rs              1.0       'Za128rs' (Reservation Set Size of at Most 128 Bytes)
    za64rs               1.0       'Za64rs' (Reservation Set Size of at Most 64 Bytes)
    zaamo                1.0       'Zaamo' (Atomic Memory Operations)
    zabha                1.0       'Zabha' (Byte and Halfword Atomic Memory Operations)
    zalrsc               1.0       'Zalrsc' (Load-Reserved/Store-Conditional)
    zama16b              1.0       'Zama16b' (Atomic 16-byte misaligned loads, stores and AMOs)
    zawrs                1.0       'Zawrs' (Wait on Reservation Set)
    zfa                  1.0       'Zfa' (Additional Floating-Point)
    zfbfmin              1.0       'Zfbfmin' (Scalar BF16 Converts)
    zfh                  1.0       'Zfh' (Half-Precision Floating-Point)
    zfhmin               1.0       'Zfhmin' (Half-Precision Floating-Point Minimal)
    zfinx                1.0       'Zfinx' (Float in Integer)
    zdinx                1.0       'Zdinx' (Double in Integer)
    zca                  1.0       'Zca' (part of the C extension, excluding compressed floating point loads/stores)
    zcb                  1.0       'Zcb' (Compressed basic bit manipulation instructions)
    zcd                  1.0       'Zcd' (Compressed Double-Precision Floating-Point Instructions)
    zce                  1.0       'Zce' (Compressed extensions for microcontrollers)
    zcf                  1.0       'Zcf' (Compressed Single-Precision Floating-Point Instructions)
    zcmop                1.0       'Zcmop' (Compressed May-Be-Operations)
    zcmp                 1.0       'Zcmp' (sequenced instructions for code-size reduction)
    zcmt                 1.0       'Zcmt' (table jump instructions for code-size reduction)
    zba                  1.0       'Zba' (Address Generation Instructions)
    zbb                  1.0       'Zbb' (Basic Bit-Manipulation)
    zbc                  1.0       'Zbc' (Carry-Less Multiplication)
    zbkb                 1.0       'Zbkb' (Bitmanip instructions for Cryptography)
    zbkc                 1.0       'Zbkc' (Carry-less multiply instructions for Cryptography)
    zbkx                 1.0       'Zbkx' (Crossbar permutation instructions)
    zbs                  1.0       'Zbs' (Single-Bit Instructions)
    zk                   1.0       'Zk' (Standard scalar cryptography extension)
    zkn                  1.0       'Zkn' (NIST Algorithm Suite)
    zknd                 1.0       'Zknd' (NIST Suite: AES Decryption)
    zkne                 1.0       'Zkne' (NIST Suite: AES Encryption)
    zknh                 1.0       'Zknh' (NIST Suite: Hash Function Instructions)
    zkr                  1.0       'Zkr' (Entropy Source Extension)
    zks                  1.0       'Zks' (ShangMi Algorithm Suite)
    zksed                1.0       'Zksed' (ShangMi Suite: SM4 Block Cipher Instructions)
    zksh                 1.0       'Zksh' (ShangMi Suite: SM3 Hash Function Instructions)
    zkt                  1.0       'Zkt' (Data Independent Execution Latency)
    ztso                 1.0       'Ztso' (Memory Model - Total Store Order)
    zvbb                 1.0       'Zvbb' (Vector basic bit-manipulation instructions)
    zvbc                 1.0       'Zvbc' (Vector Carryless Multiplication)
    zve32f               1.0       'Zve32f' (Vector Extensions for Embedded Processors with maximal 32 EEW and F extension)
    zve32x               1.0       'Zve32x' (Vector Extensions for Embedded Processors with maximal 32 EEW)
    zve64d               1.0       'Zve64d' (Vector Extensions for Embedded Processors with maximal 64 EEW, F and D extension)
    zve64f               1.0       'Zve64f' (Vector Extensions for Embedded Processors with maximal 64 EEW and F extension)
    zve64x               1.0       'Zve64x' (Vector Extensions for Embedded Processors with maximal 64 EEW)
    zvfbfmin             1.0       'Zvbfmin' (Vector BF16 Converts)
    zvfbfwma             1.0       'Zvfbfwma' (Vector BF16 widening mul-add)
    zvfh                 1.0       'Zvfh' (Vector Half-Precision Floating-Point)
    zvfhmin              1.0       'Zvfhmin' (Vector Half-Precision Floating-Point Minimal)
    zvkb                 1.0       'Zvkb' (Vector Bit-manipulation used in Cryptography)
    zvkg                 1.0       'Zvkg' (Vector GCM instructions for Cryptography)
    zvkn                 1.0       'Zvkn' (shorthand for 'Zvkned', 'Zvknhb', 'Zvkb', and 'Zvkt')
    zvknc                1.0       'Zvknc' (shorthand for 'Zvknc' and 'Zvbc')
    zvkned               1.0       'Zvkned' (Vector AES Encryption & Decryption (Single Round))
    zvkng                1.0       'zvkng' (shorthand for 'Zvkn' and 'Zvkg')
    zvknha               1.0       'Zvknha' (Vector SHA-2 (SHA-256 only))
    zvknhb               1.0       'Zvknhb' (Vector SHA-2 (SHA-256 and SHA-512))
    zvks                 1.0       'Zvks' (shorthand for 'Zvksed', 'Zvksh', 'Zvkb', and 'Zvkt')
    zvksc                1.0       'Zvksc' (shorthand for 'Zvks' and 'Zvbc')
    zvksed               1.0       'Zvksed' (SM4 Block Cipher Instructions)
    zvksg                1.0       'Zvksg' (shorthand for 'Zvks' and 'Zvkg')
    zvksh                1.0       'Zvksh' (SM3 Hash Function Instructions)
    zvkt                 1.0       'Zvkt' (Vector Data-Independent Execution Latency)
    zvl1024b             1.0       'Zvl' (Minimum Vector Length) 1024
    zvl128b              1.0       'Zvl' (Minimum Vector Length) 128
    zvl16384b            1.0       'Zvl' (Minimum Vector Length) 16384
    zvl2048b             1.0       'Zvl' (Minimum Vector Length) 2048
    zvl256b              1.0       'Zvl' (Minimum Vector Length) 256
    zvl32768b            1.0       'Zvl' (Minimum Vector Length) 32768
    zvl32b               1.0       'Zvl' (Minimum Vector Length) 32
    zvl4096b             1.0       'Zvl' (Minimum Vector Length) 4096
    zvl512b              1.0       'Zvl' (Minimum Vector Length) 512
    zvl64b               1.0       'Zvl' (Minimum Vector Length) 64
    zvl65536b            1.0       'Zvl' (Minimum Vector Length) 65536
    zvl8192b             1.0       'Zvl' (Minimum Vector Length) 8192
    zhinx                1.0       'Zhinx' (Half Float in Integer)
    zhinxmin             1.0       'Zhinxmin' (Half Float in Integer Minimal)
    shcounterenw         1.0       'Shcounterenw' (Support writeable hcounteren enable bit for any hpmcounter that is not read-only zero)
    shgatpa              1.0       'Sgatpa' (SvNNx4 mode supported for all modes supported by satp, as well as Bare)
    shtvala              1.0       'Shtvala' (htval provides all needed values)
    shvsatpa             1.0       'Svsatpa' (vsatp supports all modes supported by satp)
    shvstvala            1.0       'Shvstvala' (vstval provides all needed values)
    shvstvecd            1.0       'Shvstvecd' (vstvec supports Direct mode)
    smaia                1.0       'Smaia' (Advanced Interrupt Architecture Machine Level)
    smcdeleg             1.0       'Smcdeleg' (Counter Delegation Machine Level)
    smcsrind             1.0       'Smcsrind' (Indirect CSR Access Machine Level)
    smepmp               1.0       'Smepmp' (Enhanced Physical Memory Protection)
    smstateen            1.0       'Smstateen' (Machine-mode view of the state-enable extension)
    ssaia                1.0       'Ssaia' (Advanced Interrupt Architecture Supervisor Level)
    ssccfg               1.0       'Ssccfg' (Counter Configuration Supervisor Level)
    ssccptr              1.0       'Ssccptr' (Main memory supports page table reads)
    sscofpmf             1.0       'Sscofpmf' (Count Overflow and Mode-Based Filtering)
    sscounterenw         1.0       'Sscounterenw' (Support writeable scounteren enable bit for any hpmcounter that is not read-only zero)
    sscsrind             1.0       'Sscsrind' (Indirect CSR Access Supervisor Level)
    ssstateen            1.0       'Ssstateen' (Supervisor-mode view of the state-enable extension)
    ssstrict             1.0       'Ssstrict' (No non-conforming extensions are present)
    sstc                 1.0       'Sstc' (Supervisor-mode timer interrupts)
    sstvala              1.0       'Sstvala' (stval provides all needed values)
    sstvecd              1.0       'Sstvecd' (stvec supports Direct mode)
    ssu64xl              1.0       'Ssu64xl' (UXLEN=64 supported)
    svade                1.0       'Svade' (Raise exceptions on improper A/D bits)
    svadu                1.0       'Svadu' (Hardware A/D updates)
    svbare               1.0       'Svbare' $(satp mode Bare supported)
    svinval              1.0       'Svinval' (Fine-Grained Address-Translation Cache Invalidation)
    svnapot              1.0       'Svnapot' (NAPOT Translation Contiguity)
    svpbmt               1.0       'Svpbmt' (Page-Based Memory Types)
    xcvalu               1.0       'XCValu' (CORE-V ALU Operations)
    xcvbi                1.0       'XCVbi' (CORE-V Immediate Branching)
    xcvbitmanip          1.0       'XCVbitmanip' (CORE-V Bit Manipulation)
    xcvelw               1.0       'XCVelw' (CORE-V Event Load Word)
    xcvmac               1.0       'XCVmac' (CORE-V Multiply-Accumulate)
    xcvmem               1.0       'XCVmem' (CORE-V Post-incrementing Load & Store)
    xcvsimd              1.0       'XCVsimd' (CORE-V SIMD ALU)
    xsfcease             1.0       'XSfcease' (SiFive sf.cease Instruction)
    xsfvcp               1.0       'XSfvcp' (SiFive Custom Vector Coprocessor Interface Instructions)
    xsfvfnrclipxfqf      1.0       'XSfvfnrclipxfqf' (SiFive FP32-to-int8 Ranged Clip Instructions)
    xsfvfwmaccqqq        1.0       'XSfvfwmaccqqq' (SiFive Matrix Multiply Accumulate Instruction and 4-by-4))
    xsfvqmaccdod         1.0       'XSfvqmaccdod' (SiFive Int8 Matrix Multiplication Instructions (2-by-8 and 8-by-2))
    xsfvqmaccqoq         1.0       'XSfvqmaccqoq' (SiFive Int8 Matrix Multiplication Instructions (4-by-8 and 8-by-4))
    xsifivecdiscarddlone 1.0       'XSiFivecdiscarddlone' (SiFive sf.cdiscard.d.l1 Instruction)
    xsifivecflushdlone   1.0       'XSiFivecflushdlone' (SiFive sf.cflush.d.l1 Instruction)
    xtheadba             1.0       'XTHeadBa' (T-Head address calculation instructions)
    xtheadbb             1.0       'XTHeadBb' (T-Head basic bit-manipulation instructions)
    xtheadbs             1.0       'XTHeadBs' (T-Head single-bit instructions)
    xtheadcmo            1.0       'XTHeadCmo' (T-Head cache management instructions)
    xtheadcondmov        1.0       'XTHeadCondMov' (T-Head conditional move instructions)
    xtheadfmemidx        1.0       'XTHeadFMemIdx' (T-Head FP Indexed Memory Operations)
    xtheadmac            1.0       'XTHeadMac' (T-Head Multiply-Accumulate Instructions)
    xtheadmemidx         1.0       'XTHeadMemIdx' (T-Head Indexed Memory Operations)
    xtheadmempair        1.0       'XTHeadMemPair' (T-Head two-GPR Memory Operations)
    xtheadsync           1.0       'XTHeadSync' (T-Head multicore synchronization instructions)
    xtheadvdot           1.0       'XTHeadVdot' (T-Head Vector Extensions for Dot)
    xventanacondops      1.0       'XVentanaCondOps' (Ventana Conditional Ops)
    xwchc                2.2       'Xwchc' (WCH/QingKe additional compressed opcodes)";

    let mut sorted_exts = march_help_exts
        .split("\n")
        .map(|line| line.split_whitespace().next().unwrap())
        .collect::<Vec<_>>();
    sorted_exts.sort();

    let mut sorted_llvm_exts = llvm_print_supported_exts
        .split("\n")
        .map(|line| line.split_whitespace().next().unwrap())
        .collect::<Vec<_>>();
    sorted_llvm_exts.sort();

    sorted_exts.append(&mut sorted_llvm_exts);

    let mut set = HashSet::new();

    sorted_exts.retain(|x| set.insert(*x));

    sorted_llvm_exts.sort();

    println!("struct March {{");
    for ext in sorted_exts {
        println!("\t{ext}: bool,");
    }
    println!("}}");
}
