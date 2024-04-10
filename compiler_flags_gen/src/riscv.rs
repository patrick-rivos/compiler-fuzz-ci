use std::fmt;

use arbitrary::Arbitrary;
use struct_iterable::Iterable;

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct March {
    mabi: Mabi,
    pub g: bool,
    i: bool,
    // e: bool, // Error: rv64e requires lp64e ABI
    pub m: bool,
    a: bool,
    f: bool,
    d: bool,
    c: bool,
    h: bool,
    pub v: bool,
    svinval: bool,
    svnapot: bool,
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
    zawrs: bool,
    zba: bool,
    zbb: bool,
    zbc: bool,
    zbkb: bool,
    zbkc: bool,
    zbkx: bool,
    zbs: bool,
    zdinx: bool,
    zfh: bool,
    zfhmin: bool,
    zfinx: bool,
    zhinx: bool,
    zhinxmin: bool,
    zicbom: bool,
    zicbop: bool,
    zicboz: bool,
    zicond: bool,
    zicsr: bool,
    zifencei: bool,
    zihintntl: bool,
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
    pub ztso0p1: bool,
    pub zvbb: bool,
    pub zvbc: bool,
    pub zve32f: bool,
    pub zve32x: bool,
    pub zve64d: bool,
    pub zve64f: bool,
    pub zve64x: bool,
    pub zvfh: bool,
    pub zvfhmin: bool,
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
    zvl: Zvl,
}

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Clone, Copy)]
enum Mabi {
    // ilp32, // Linker against module fail
    // ilp32d, // Linker against module fail
    // ilp32e, // Linker against module fail
    // ilp32f, // Linker against module fail
    // lp64, // Linker against module fail
    lp64d,
    // lp64e, // Linker against module fail
    // lp64f, // Linker against module fail
}

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Clone, Copy, PartialEq)]
enum Zvl {
    zvl32b, // Exec fail
    zvl64b, // Exec fail
    zvl128b,
    zvl256b,
    zvl512b,
    zvl1024b,
    zvl2048b, // Exec fail
    zvl4096b, // Exec fail
    //     zvl8192b,
    //     zvl16384b,
    //     zvl32768b,
    //     zvl65536b,
    nozvl,
}

impl March {
    pub fn sanitize(&mut self) {
        // Use i by default
        self.i = true;

        // G disables all extensions it represents
        self.i = self.i && (!self.g);
        self.m = self.m && (!self.g);
        self.a = self.a && (!self.g);
        self.f = self.f && (!self.g);
        self.d = self.d && (!self.g);
        self.zifencei = self.zifencei && (!self.g);
        self.zicsr = self.zicsr && (!self.g);

        self.h = self.h && self.i;
        self.d = (self.d
            || (match self.mabi {
                // Mabi::ilp32 => false,
                // Mabi::ilp32d => true,
                // Mabi::ilp32e => false,
                // Mabi::ilp32f => true,
                // Mabi::lp64 => false,
                Mabi::lp64d => true,
                // Mabi::lp64e => false,
                // Mabi::lp64f => false,
            }))
            && (!self.g);
        self.f = (self.f
            || (match self.mabi {
                // Mabi::ilp32 => false,
                // Mabi::ilp32d => false,
                // Mabi::ilp32e => false,
                // Mabi::ilp32f => true,
                // Mabi::lp64 => false,
                Mabi::lp64d => false,
                // Mabi::lp64e => false,
                // Mabi::lp64f => true,
            }))
            && (!self.g);
        self.zfinx = self.zfinx && !self.f && !self.d && !self.g;
        self.zhinxmin = self.zhinxmin && self.zfinx;
        self.zdinx = self.zdinx && self.zfinx;
        self.zhinx = self.zhinx && self.zfinx;
        self.zhinx = self.zhinx && self.zhinxmin;

        // TODO: Add zve
        self.v = self.v
            || (self.zvbb
                || self.zvbc
                || self.zve32f
                || self.zve32x
                || self.zve64d
                || self.zve64f
                || self.zve64x
                || self.zvl != Zvl::nozvl)
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
                    match value {
                        // Mabi::ilp32 => "rv32".to_string(),
                        // Mabi::ilp32d => "rv32".to_string(),
                        // Mabi::ilp32e => "rv32".to_string(),
                        // Mabi::ilp32f => "rv32".to_string(),
                        // Mabi::lp64 => "rv64".to_string(),
                        Mabi::lp64d => "rv64".to_string(),
                        // Mabi::lp64e => "rv64".to_string(),
                        // Mabi::lp64f => "rv64".to_string(),
                    }
                } else {
                    panic!("Unknown datatype for field: {}", field_name)
                }
            })
            .collect::<Vec<_>>();

        let abi = match abi {
            // Some(Mabi::ilp32) => "ilp32",
            // Some(Mabi::ilp32d) => "ilp32d",
            // Some(Mabi::ilp32e) => "ilp32e",
            // Some(Mabi::ilp32f) => "ilp32f",
            // Some(Mabi::lp64) => "lp64",
            Some(Mabi::lp64d) => "lp64d",
            // Some(Mabi::lp64e) => "lp64e",
            // Some(Mabi::lp64f) => "lp64f",
            None => panic!(),
        };

        write!(f, "-march={} -mabi={}", flags.join(""), abi)
    }
}

fn _convert_extensions_to_struct() {
    let exts = "
  e
  i
  m
  a
  f
  d
  c
  h
  v
  zicsr
  zifencei
  zicond
  zawrs
  zba
  zbb
  zbc
  zbs
  zfinx
  zdinx
  zhinx
  zhinxmin
  zbkb
  zbkc
  zbkx
  zkne
  zknd
  zknh
  zkr
  zksed
  zksh
  zkt
  zihintntl
  zicboz
  zicbom
  zicbop
  zk
  zkn
  zks
  ztso
  zve32x
  zve32f
  zve64x
  zve64f
  zve64d
  zvbb
  zvbc
  zvkg
  zvkned
  zvknha
  zvknhb
  zvksed
  zvksh
  zvkn
  zvknc
  zvkng
  zvks
  zvksc
  zvksg
  zvkt
  zvl32b
  zvl64b
  zvl128b
  zvl256b
  zvl512b
  zvl1024b
  zvl2048b
  zvl4096b
  zvl8192b
  zvl16384b
  zvl32768b
  zvl65536b
  zfh
  zfhmin
  zvfhmin
  zvfh
  zmmul
  svinval
  svnapot
  xtheadba
  xtheadbb
  xtheadbs
  xtheadcmo
  xtheadcondmov
  xtheadfmemidx
  xtheadfmv
  xtheadint
  xtheadmac
  xtheadmemidx
  xtheadmempair
  xtheadsync
  ";

    let mut sorted_exts = exts.split_whitespace().collect::<Vec<_>>();
    sorted_exts.sort();

    println!("struct March {{");
    for ext in sorted_exts {
        println!("\t{ext}: bool,");
    }
    println!("}}");
}
