use log::trace;

pub fn parse_riscv() {
    let _riscv_opts = "
    -mbranch-cost=@var{N-instruction}
    -mplt  -mno-plt
    -mabi=@var{ABI-string}
    -mfdiv  -mno-fdiv
    -mdiv  -mno-div
    -misa-spec=@var{ISA-spec-string}
    -march=@var{ISA-string}
    -mtune=@var{processor-string}
    -mpreferred-stack-boundary=@var{num}
    -msmall-data-limit=@var{N-bytes}
    -msave-restore  -mno-save-restore
    -mshorten-memrefs  -mno-shorten-memrefs
    -mstrict-align  -mno-strict-align
    -mcmodel=medlow  -mcmodel=medany
    -mexplicit-relocs  -mno-explicit-relocs
    -mrelax  -mno-relax
    -mriscv-attribute  -mno-riscv-attribute
    -malign-data=@var{type}
    -mbig-endian  -mlittle-endian
    -mstack-protector-guard=@var{guard}  -mstack-protector-guard-reg=@var{reg}
    -mstack-protector-guard-offset=@var{offset}
    -mcsr-check -mno-csr-check
    -mmovcc  -mno-movcc
    -minline-atomics  -mno-inline-atomics
    -minline-strlen  -mno-inline-strlen
    -minline-strcmp  -mno-inline-strcmp
    -minline-strncmp  -mno-inline-strncmp
    ";

    let _optimization_opts = "
	-faggressive-loop-optimizations
    -falign-functions[=@var{n}[:@var{m}:[@var{n2}[:@var{m2}]]]]
    -falign-jumps[=@var{n}[:@var{m}:[@var{n2}[:@var{m2}]]]]
    -falign-labels[=@var{n}[:@var{m}:[@var{n2}[:@var{m2}]]]]
    -falign-loops[=@var{n}[:@var{m}:[@var{n2}[:@var{m2}]]]]
    -fmin-function-alignment=[@var{n}]
    -fno-allocation-dce -fallow-store-data-races
    -fassociative-math  -fauto-profile  -fauto-profile[=@var{path}]
    -fauto-inc-dec  -fbranch-probabilities
    -fcaller-saves
    -fcombine-stack-adjustments  -fconserve-stack
    -ffold-mem-offsets
    -fcompare-elim  -fcprop-registers  -fcrossjumping
    -fcse-follow-jumps  -fcse-skip-blocks  -fcx-fortran-rules
    -fcx-limited-range
    -fdata-sections  -fdce  -fdelayed-branch
    -fdelete-null-pointer-checks  -fdevirtualize  -fdevirtualize-speculatively
    -fdevirtualize-at-ltrans  -fdse
    -fearly-inlining  -fipa-sra  -fexpensive-optimizations  -ffat-lto-objects
    -ffast-math  -ffinite-math-only  -ffloat-store  -fexcess-precision=@var{style}
    -ffinite-loops
    -fforward-propagate  -ffp-contract=@var{style}  -ffunction-sections
    -fgcse  -fgcse-after-reload  -fgcse-las  -fgcse-lm  -fgraphite-identity
    -fgcse-sm  -fhoist-adjacent-loads  -fif-conversion
    -fif-conversion2  -findirect-inlining
    -finline-stringops[=@var{fn}]
    -finline-functions  -finline-functions-called-once  -finline-limit=@var{n}
    -finline-small-functions -fipa-modref -fipa-cp  -fipa-cp-clone
    -fipa-bit-cp  -fipa-vrp  -fipa-pta  -fipa-profile  -fipa-pure-const
    -fipa-reference  -fipa-reference-addressable
    -fipa-stack-alignment  -fipa-icf  -fira-algorithm=@var{algorithm}
    -flive-patching=@var{level}
    -fira-region=@var{region}  -fira-hoist-pressure
    -fira-loop-pressure  -fno-ira-share-save-slots
    -fno-ira-share-spill-slots
    -fisolate-erroneous-paths-dereference  -fisolate-erroneous-paths-attribute
    -fivopts  -fkeep-inline-functions  -fkeep-static-functions
    -fkeep-static-consts  -flimit-function-alignment  -flive-range-shrinkage
    -floop-block  -floop-interchange  -floop-strip-mine
    -floop-unroll-and-jam  -floop-nest-optimize
    -floop-parallelize-all  -flra-remat  -flto  -flto-compression-level=@var{n}
    -flto-partition=@var{alg}  -fmerge-all-constants
    -fmerge-constants  -fmodulo-sched  -fmodulo-sched-allow-regmoves
    -fmove-loop-invariants  -fmove-loop-stores  -fno-branch-count-reg
    -fno-defer-pop  -fno-fp-int-builtin-inexact  -fno-function-cse
    -fno-guess-branch-probability  -fno-inline  -fno-math-errno  -fno-peephole
    -fno-peephole2  -fno-printf-return-value  -fno-sched-interblock
    -fno-sched-spec  -fno-signed-zeros
    -fno-toplevel-reorder  -fno-trapping-math  -fno-zero-initialized-in-bss
    -fomit-frame-pointer  -foptimize-sibling-calls
    -fpartial-inlining  -fpeel-loops  -fpredictive-commoning
    -fprefetch-loop-arrays
    -fprofile-correction
    -fprofile-use  -fprofile-use=@var{path} -fprofile-partial-training
    -fprofile-values -fprofile-reorder-functions
    -freciprocal-math  -free  -frename-registers  -freorder-blocks
    -freorder-blocks-algorithm=@var{algorithm}
    -freorder-blocks-and-partition  -freorder-functions
    -frerun-cse-after-loop  -freschedule-modulo-scheduled-loops
    -frounding-math  -fsave-optimization-record
    -fsched2-use-superblocks  -fsched-pressure
    -fsched-spec-load  -fsched-spec-load-dangerous
    -fsched-stalled-insns-dep[=@var{n}]  -fsched-stalled-insns[=@var{n}]
    -fsched-group-heuristic  -fsched-critical-path-heuristic
    -fsched-spec-insn-heuristic  -fsched-rank-heuristic
    -fsched-last-insn-heuristic  -fsched-dep-count-heuristic
    -fschedule-fusion
    -fschedule-insns  -fschedule-insns2  -fsection-anchors
    -fselective-scheduling  -fselective-scheduling2
    -fsel-sched-pipelining  -fsel-sched-pipelining-outer-loops
    -fsemantic-interposition  -fshrink-wrap  -fshrink-wrap-separate
    -fsignaling-nans
    -fsingle-precision-constant  -fsplit-ivs-in-unroller  -fsplit-loops
    -fsplit-paths
    -fsplit-wide-types  -fsplit-wide-types-early  -fssa-backprop  -fssa-phiopt
    -fstdarg-opt  -fstore-merging  -fstrict-aliasing -fipa-strict-aliasing
    -fthread-jumps  -ftracer  -ftree-bit-ccp
    -ftree-builtin-call-dce  -ftree-ccp  -ftree-ch
    -ftree-coalesce-vars  -ftree-copy-prop  -ftree-dce  -ftree-dominator-opts
    -ftree-dse  -ftree-forwprop  -ftree-fre  -fcode-hoisting
    -ftree-loop-if-convert  -ftree-loop-im
    -ftree-phiprop  -ftree-loop-distribution  -ftree-loop-distribute-patterns
    -ftree-loop-ivcanon  -ftree-loop-linear  -ftree-loop-optimize
    -ftree-loop-vectorize
    -ftree-parallelize-loops=@var{n}  -ftree-pre  -ftree-partial-pre  -ftree-pta
    -ftree-reassoc  -ftree-scev-cprop  -ftree-sink  -ftree-slsr  -ftree-sra
    -ftree-switch-conversion  -ftree-tail-merge
    -ftree-ter  -ftree-vectorize  -ftree-vrp  -ftrivial-auto-var-init=@var{choice}
    -funconstrained-commons -funit-at-a-time  -funroll-all-loops
    -funroll-loops -funsafe-math-optimizations  -funswitch-loops
    -fipa-ra  -fvariable-expansion-in-unroller  -fvect-cost-model  -fvpt
    -fweb  -fwhole-program  -fwpa  -fuse-linker-plugin -fzero-call-used-regs=@var{choice}
    --param @var{name}=@var{value}
    -O  -O0  -O1  -O2  -O3  -Os  -Ofast  -Og  -Oz
	";

    let _debug_opts = "
	-g  -g@var{level}  -gdwarf  -gdwarf-@var{version}
	-gbtf -gctf  -gctf@var{level}
	-ggdb  -grecord-gcc-switches  -gno-record-gcc-switches
	-gstrict-dwarf  -gno-strict-dwarf
	-gas-loc-support  -gno-as-loc-support
	-gas-locview-support  -gno-as-locview-support
	-gcodeview
	-gcolumn-info  -gno-column-info  -gdwarf32  -gdwarf64
	-gstatement-frontiers  -gno-statement-frontiers
	-gvariable-location-views  -gno-variable-location-views
	-ginternal-reset-location-views  -gno-internal-reset-location-views
	-ginline-points  -gno-inline-points
	-gvms -gz@r{[}=@var{type}@r{]}
	-gsplit-dwarf  -gdescribe-dies  -gno-describe-dies
	-fdebug-prefix-map=@var{old}=@var{new}  -fdebug-types-section
	-fno-eliminate-unused-debug-types
	-femit-struct-debug-baseonly  -femit-struct-debug-reduced
	-femit-struct-debug-detailed@r{[}=@var{spec-list}@r{]}
	-fno-eliminate-unused-debug-symbols  -femit-class-debug-always
	-fno-merge-debug-strings  -fno-dwarf2-cfi-asm
	-fvar-tracking  -fvar-tracking-assignments
	";

    let dev_opts = "
	-d@var{letters}  -dumpspecs  -dumpmachine  -dumpversion
	-dumpfullversion  -fcallgraph-info@r{[}=su,da@r{]}
	-fchecking  -fchecking=@var{n}
	-fdbg-cnt-list  -fdbg-cnt=@var{counter-value-list}
	-fdisable-ipa-@var{pass_name}
	-fdisable-rtl-@var{pass_name}
	-fdisable-rtl-@var{pass-name}=@var{range-list}
	-fdisable-tree-@var{pass_name}
	-fdisable-tree-@var{pass-name}=@var{range-list}
	-fdump-debug  -fdump-earlydebug
	-fdump-noaddr  -fdump-unnumbered  -fdump-unnumbered-links
	-fdump-final-insns@r{[}=@var{file}@r{]}
	-fdump-ipa-all  -fdump-ipa-cgraph  -fdump-ipa-inline
	-fdump-lang-all
	-fdump-lang-@var{switch}
	-fdump-lang-@var{switch}-@var{options}
	-fdump-lang-@var{switch}-@var{options}=@var{filename}
	-fdump-passes
	-fdump-rtl-@var{pass}  -fdump-rtl-@var{pass}=@var{filename}
	-fdump-statistics
	-fdump-tree-all
	-fdump-tree-@var{switch}
	-fdump-tree-@var{switch}-@var{options}
	-fdump-tree-@var{switch}-@var{options}=@var{filename}
	-fcompare-debug@r{[}=@var{opts}@r{]}  -fcompare-debug-second
	-fenable-@var{kind}-@var{pass}
	-fenable-@var{kind}-@var{pass}=@var{range-list}
	-fira-verbose=@var{n}
	-flto-report  -flto-report-wpa  -fmem-report-wpa
	-fmem-report  -fpre-ipa-mem-report  -fpost-ipa-mem-report
	-fopt-info  -fopt-info-@var{options}@r{[}=@var{file}@r{]}
	-fmultiflags  -fprofile-report
	-frandom-seed=@var{string}  -fsched-verbose=@var{n}
	-fstats  -fstack-usage  -ftime-report  -ftime-report-details
	-fvar-tracking-assignments-toggle  -gtoggle
	-print-file-name=@var{library}  -print-libgcc-file-name
	-print-multi-directory  -print-multi-lib  -print-multi-os-directory
	-print-prog-name=@var{program}  -print-search-dirs  -Q
	-print-sysroot  -print-sysroot-headers-suffix
	-save-temps  -save-temps=cwd  -save-temps=obj  -time@r{[}=@var{file}@r{]}
	";

    let mut sorted_opts = dev_opts.split_whitespace().collect::<Vec<_>>();
    sorted_opts.sort();
    sorted_opts.dedup();

    let mut toggleable = sorted_opts
        .iter()
        .filter(|opt| {
            if !opt.starts_with("-f") && !opt.starts_with("-m") && !opt.starts_with("-g") {
                trace!("Opt: {} not toggleable", opt);
                return false;
            }
            if opt.contains('=') {
                trace!("Opt: {} needs arg", opt);
                return false;
            }
            if opt.contains('{') {
                trace!("Opt: {} is enum", opt);
                return false;
            }

            true
        })
        .map(|x| x.replacen("-fno-", "-f", 1))
        .map(|x| x.replacen("-mno-", "-m", 1))
        .map(|x| x.replacen("-gno-", "-g", 1))
        .collect::<Vec<_>>();
    toggleable.sort();
    toggleable.dedup();

    println!("struct GccFlags {{");
    for toggle_opt in &toggleable {
        println!(
            "\t{}: ToggleOpt,",
            toggle_opt.replacen('-', "", 1).replace('-', "_")
        );
    }
    println!("}}");

    trace!("{}", toggleable.join(" "));

    let mut enums = sorted_opts
        .iter()
        .filter(|opt| {
            if opt.contains('=') {
                trace!("Opt: {} needs arg", opt);
                return false;
            }
            if !opt.contains('{') {
                trace!("Opt: {} is not enum", opt);
                return false;
            }
            true
        })
        .collect::<Vec<_>>();

    enums.sort();
    enums.dedup();

    for enum_opt in &enums {
        println!("\t{}: ,", enum_opt.replacen('-', "", 1).replace('-', "_"));
    }
}
