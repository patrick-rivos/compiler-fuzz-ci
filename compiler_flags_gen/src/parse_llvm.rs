pub fn parse_llvm() {
    let binding = llvm_help_hidden_message();
    let opts = binding.lines();

    let mut sorted_opts = opts.into_iter().collect::<Vec<_>>();
    sorted_opts.sort();
    sorted_opts.dedup();

    let mut toggleable = sorted_opts
        .iter()
        .filter_map(|line| {
            let opt = line.trim_start();

            if !opt.starts_with('-') {
                // Not an option
                return None;
            }
            if opt.contains("(AArch64 only)")
                || opt.contains("(AMDGPU only)")
                || opt.contains("(ARM only)")
                || opt.contains("(SystemZ only)")
                || opt.contains("(M68k only)")
                || opt.contains("(SPARC only)")
                || opt.contains("(Hexagon only)")
                || opt.contains("(HIP only)")
                || opt.contains("(x86 only)")
                || opt.contains("(x86/SystemZ only)")
                || opt.contains("(AArch32/MIPSr6 only)")
                || opt.contains("(Mips only)")
                || opt.contains("(AIX PPC64 only)")
                || opt.contains("(MIPS only)")
                || opt.contains("(PPC32 only)")
                || opt.contains("(ARM/Mips only)")
                || opt.contains("(AArch64/x86 only)")
                || opt.contains("(AIX only)")
                || opt.contains("(LASX)")
                || opt.contains("(MIPS)")
                || opt.contains("(SESES)")
                || opt.contains("(LVI)")
                || opt.contains("(LSX)")
                || opt.contains("(LSX)")
                || opt.contains("(LSX)")
                || opt.contains("(LSX)")
                || opt.contains("(LSX)")
                || opt.contains("(LSX)")
            {
                // Different arch
                return None;
            }

            if opt.contains('<') && opt.contains('>') {
                // Positional argument
                return None;
            }

            let opt = opt.split_whitespace().next().unwrap();

            if !opt.starts_with("-f") && !opt.starts_with("-m") && !opt.starts_with("-g") {
                // Not toggleable
                return None;
            }
            if opt.contains('=') {
                // Positional argument
                return None;
            }
            if opt.contains('+') {
                // Positional argument
                return None;
            }

            Some(opt)
        })
        .map(|x| x.replacen("-fno-", "-f", 1))
        .map(|x| x.replacen("-mno-", "-m", 1))
        .map(|x| x.replacen("-gno-", "-g", 1))
        .collect::<Vec<_>>();
    toggleable.sort();
    toggleable.dedup();

    println!("struct LlvmFlags {{");
    for toggle_opt in &toggleable {
        println!(
            "\t{}: ToggleOpt,",
            toggle_opt.replacen('-', "", 1).replace('-', "_")
        );
    }
    println!("}}");
}

fn llvm_help_hidden_message() -> String {
    "
OVERVIEW: clang LLVM compiler

USAGE: clang-19 [options] file...

DEBUG/DEVELOPMENT OPTIONS:
  -ccc-install-dir <value>
                      Simulate installation in the given directory
  -ccc-print-bindings Show bindings of tools to actions
  -gen-cdb-fragment-path <value>
                      Emit a compilation database fragment to the specified directory
  -gen-reproducer     Auto-generates preprocessed source files and a reproduction script
  -via-file-asm       Write assembly to file for input to assemble jobs

DRIVER OPTIONS:
  -ccc-arcmt-check      Check for ARC migration issues that need manual handling
  -ccc-arcmt-migrate <value>
                        Apply modifications and produces temporary files that conform to ARC
  -ccc-arcmt-modify     Apply modifications to files to conform to ARC
  -ccc-gcc-name <gcc-path>
                        Name for native GCC compiler
  -ccc-objcmt-migrate <value>
                        Apply modifications and produces temporary files to migrate to modern ObjC syntax
  --driver-mode=<value> Set the driver mode to either 'gcc', 'g++', 'cpp', 'cl' or 'flang'
  --rsp-quoting=<value> Set the rsp quoting to either 'posix', or 'windows'

OPTIONS:
  -###                    Print (but do not run) the commands to run for this compilation
  --amdgpu-arch-tool=<value>
                          Tool used for detecting AMD GPU arch in the system.
  --analyzer-output <value>
                          Static analyzer report output format (html|plist|plist-multi-file|plist-html|sarif|sarif-html|text).
  --analyze               Run the static analyzer
  -arcmt-migrate-emit-errors
                          Emit ARC errors even if the migrator can fix them
  -arcmt-migrate-report-output <value>
                          Output path for the plist report
  -B <prefix>             Search $prefix$file for executables, libraries, and data files. If $prefix is a directory, search $prefix/$file
  -b <arg>                Pass -b <arg> to the linker on AIX
  -canonical-prefixes     Use absolute paths for invoking subcommands (default)
  -ccc-print-phases       Dump list of actions to perform
  -CC                     Include comments from within macros in preprocessed output
  -cl-denorms-are-zero    OpenCL only. Allow denormals to be flushed to zero.
  -cl-ext=<value>         OpenCL only. Enable or disable OpenCL extensions/optional features. The argument is a comma-separated sequence of one or more extension names, each prefixed by '+' or '-'.
  -cl-fast-relaxed-math   OpenCL only. Sets -cl-finite-math-only and -cl-unsafe-math-optimizations, and defines __FAST_RELAXED_MATH__.
  -cl-finite-math-only    OpenCL only. Allow floating-point optimizations that assume arguments and results are not NaNs or +-Inf.
  -cl-fp32-correctly-rounded-divide-sqrt
                          OpenCL only. Specify that single precision floating-point divide and sqrt used in the program source are correctly rounded.
  -cl-kernel-arg-info     OpenCL only. Generate kernel argument metadata.
  -cl-mad-enable          OpenCL only. Allow use of less precise MAD computations in the generated binary.
  -cl-no-signed-zeros     OpenCL only. Allow use of less precise no signed zeros computations in the generated binary.
  -cl-no-stdinc           OpenCL only. Disables all standard includes containing non-native compiler types and functions.
  -cl-opt-disable         OpenCL only. This option disables all optimizations. By default optimizations are enabled.
  -cl-single-precision-constant
                          OpenCL only. Treat double precision floating-point constant as single precision constant.
  -cl-std=<value>         OpenCL language standard to compile for.
  -cl-strict-aliasing     OpenCL only. This option is added for compatibility with OpenCL 1.0.
  -cl-uniform-work-group-size
                          OpenCL only. Defines that the global work-size be a multiple of the work-group size specified to clEnqueueNDRangeKernel
  -cl-unsafe-math-optimizations
                          OpenCL only. Allow unsafe floating-point optimizations.  Also implies -cl-no-signed-zeros and -cl-mad-enable.
  --config-system-dir=<value>
                          System directory for configuration files
  --config-user-dir=<value>
                          User directory for configuration files
  --config=<file>         Specify configuration file
  --cuda-compile-host-device
                          Compile CUDA code for both host and device (default). Has no effect on non-CUDA compilations.
  --cuda-device-only      Compile CUDA code for device only
  --cuda-feature=<value>  Manually specify the CUDA feature to use
  --cuda-host-only        Compile CUDA code for host only. Has no effect on non-CUDA compilations.
  --cuda-include-ptx=<value>
                          Include PTX for the following GPU architecture (e.g. sm_35) or 'all'. May be specified more than once.
  --cuda-noopt-device-debug
                          Enable device-side debug info generation. Disables ptxas optimizations.
  --cuda-path-ignore-env  Ignore environment variables to detect CUDA installation
  --cuda-path=<value>     CUDA installation path
  -cuid=<value>           An ID for compilation unit, which should be the same for the same compilation unit but different for different compilation units. It is used to externalize device-side static variables for single source offloading languages CUDA and HIP so that they can be accessed by the host code of the same compilation unit.
  -cxx-isystem <directory>
                          Add directory to the C++ SYSTEM include search path
  -C                      Include comments in preprocessed output
  -c                      Only run preprocess, compile, and assemble steps
  -darwin-target-variant-triple <value>
                          Specify the darwin target variant triple
  -darwin-target-variant <value>
                          Generate code for an additional runtime variant of the deployment target
  -dD                     Print macro definitions in -E mode in addition to normal output
  -dependency-dot <value> Filename to write DOT-formatted header dependencies to
  -dependency-file <value>
                          Filename (or -) to write dependency output to
  -dI                     Print include directives in -E mode in addition to normal output
  -dM                     Print macro definitions in -E mode instead of normal output
  -dsym-dir <dir>         Directory to output dSYM's (if any) to
  -dumpdir <dumppfx>      Use <dumpfpx> as a prefix to form auxiliary and dump file names
  -dumpmachine            Display the compiler's target processor
  -dumpversion            Display the version of the compiler
  -D <macro>=<value>      Define <macro> to <value> (or 1 if <value> omitted)
  -emit-ast               Emit Clang AST files for source inputs
  --emit-extension-symbol-graphs
                          Generate additional symbol graphs for extended modules.
  -emit-interface-stubs   Generate Interface Stub Files.
  -emit-llvm              Use the LLVM representation for assembler and object files
  -emit-merged-ifs        Generate Interface Stub Files, emit merged text not binary.
  --emit-static-lib       Enable linker job to emit a static library.
  -emit-symbol-graph      Generate Extract API information as a side effect of compilation.
  --end-no-unused-arguments
                          Start emitting warnings for unused driver arguments
  --extract-api-ignores=<value>
                          Comma separated list of files containing a new line separated list of API symbols to ignore when extracting API information.
  -extract-api            Extract API information
  -E                      Only run the preprocessor
  -faapcs-bitfield-load   Follows the AAPCS standard that all volatile bit-field write generates at least one load. (ARM only).
  -faapcs-bitfield-width  Follow the AAPCS standard requirement stating that volatile bit-field width is dictated by the field container type. (ARM only).
  -faddrsig               Emit an address-significance table
  -falign-loops=<N>       N must be a power of two. Align loops to the boundary
  -faligned-allocation    Enable C++17 aligned allocation functions
  -fallow-editor-placeholders
                          Treat editor placeholders as valid source code
  -faltivec-src-compat=<value>
                          Source-level compatibility for Altivec vectors (for PowerPC targets). This includes results of vector comparison (scalar for 'xl', vector for 'gcc') as well as behavior when initializing with a scalar (splatting for 'xl', element zero only for 'gcc'). For 'mixed', the compatibility is as 'gcc' for 'vector bool/vector pixel' and as 'xl' for other types. Current default is 'mixed'.
  -fansi-escape-codes     Use ANSI escape codes for diagnostics
  -fapinotes-modules      Enablemodule-based external API notes support
  -fapinotes-swift-version=<version>
                          Specify the Swift version to use when filtering API notes
  -fapinotes              Enableexternal API notes support
  -fapple-kext            Use Apple's kernel extensions ABI
  -fapple-link-rtlib      Force linking the clang builtins runtime library
  -fapple-pragma-pack     Enable Apple gcc-compatible #pragma pack handling
  -fapplication-extension Restrict code to those available for App Extensions
  -fapprox-func           Allow certain math function calls to be replaced with an approximately equivalent calculation
  -fassume-nothrow-exception-dtor
                          Assume that exception objects' destructors are non-throwing
  -fasync-exceptions      Enable EH Asynchronous exceptions
  -fauto-import           MinGW specific. Enable code generation support for automatic dllimport, and enable support for it in the linker. Enabled by default.
  -fbasic-block-address-map
                          Emit the basic block address map section.
  -fbasic-block-sections=<value>
                          Place each function's basic blocks in unique sections (ELF Only)
  -fbinutils-version=<major.minor>
                          Produced object files can use all ELF features supported by this binutils version and newer. If -fno-integrated-as is specified, the generated assembly will consider GNU as support. 'none' means that all ELF features can be used, regardless of binutils support. Defaults to 2.26.
  -fblocks                Enable the 'blocks' language feature
  -fborland-extensions    Accept non-standard constructs supported by the Borland compiler
  -fbuild-session-file=<file>
                          Use the last modification time of <file> as the build session timestamp
  -fbuild-session-timestamp=<time since Epoch in seconds>
                          Time when the current build session started
  -fbuiltin-module-map    Load the clang builtins module map file.
  -fc++-abi=<value>       C++ ABI to use. This will override the target C++ ABI.
  -fcall-saved-x10        Make the x10 register call-saved (AArch64 only)
  -fcall-saved-x11        Make the x11 register call-saved (AArch64 only)
  -fcall-saved-x12        Make the x12 register call-saved (AArch64 only)
  -fcall-saved-x13        Make the x13 register call-saved (AArch64 only)
  -fcall-saved-x14        Make the x14 register call-saved (AArch64 only)
  -fcall-saved-x15        Make the x15 register call-saved (AArch64 only)
  -fcall-saved-x18        Make the x18 register call-saved (AArch64 only)
  -fcall-saved-x8         Make the x8 register call-saved (AArch64 only)
  -fcall-saved-x9         Make the x9 register call-saved (AArch64 only)
  -fcaret-diagnostics-max-lines=<value>
                          Set the maximum number of source lines to show in a caret diagnostic (0 = no limit).
  -fcf-protection=<value> Instrument control-flow architecture protection
  -fcf-protection         Enable cf-protection in 'full' mode
  -fchar8_t               Enable C++ builtin type char8_t
  -fcheck-new             Do not assume C++ operator new may not return NULL
  -fclang-abi-compat=<version>
                          Attempt to match the ABI of Clang <version>
  -fcolor-diagnostics     Enable colors in diagnostics
  -fcomment-block-commands=<arg>
                          Treat each comma separated argument in <arg> as a documentation comment block command
  -fcommon                Place uninitialized global variables in a common block
  -fcomplete-member-pointers
                          Require member pointer base types to be complete if they would be significant under the Microsoft ABI
  -fconstexpr-backtrace-limit=<value>
                          Set the maximum number of entries to print in a constexpr evaluation backtrace (0 = no limit)
  -fconstexpr-depth=<value>
                          Set the maximum depth of recursive constexpr function calls
  -fconstexpr-steps=<value>
                          Set the maximum number of steps in constexpr function evaluation
  -fcoro-aligned-allocation
                          Prefer aligned allocation for C++ Coroutines
  -fcoroutines            Enable support for the C++ Coroutines
  -fcoverage-compilation-dir=<value>
                          The compilation directory to embed in the coverage mapping.
  -fcoverage-mapping      Generate coverage mapping to enable code coverage analysis
  -fcoverage-mcdc         Enable MC/DC criteria when generating code coverage
  -fcoverage-prefix-map=<old>=<new>
                          remap file source paths <old> to <new> in coverage mapping. If there are multiple options, prefix replacement is applied in reverse order starting from the last one
  -fcrash-diagnostics-dir=<dir>
                          Put crash-report files in <dir>
  -fcrash-diagnostics=<value>
                          Set level of crash diagnostic reporting, (option: off, compiler, all)
  -fcrash-diagnostics     Enable crash diagnostic reporting (default)
  -fcs-profile-generate=<directory>
                          Generate instrumented code to collect context sensitive execution counts into <directory>/default.profraw (overridden by LLVM_PROFILE_FILE env var)
  -fcs-profile-generate   Generate instrumented code to collect context sensitive execution counts into default.profraw (overridden by LLVM_PROFILE_FILE env var)
  -fcuda-short-ptr        Use 32-bit pointers for accessing const/local/shared address spaces
  -fcx-fortran-rules      Range reduction is enabled for complex arithmetic operations.
  -fcx-limited-range      Basic algebraic expansions of complex arithmetic operations involving are enabled.
  -fcxx-exceptions        Enable C++ exceptions
  -fcxx-modules           Enable modules for C++
  -fdata-sections         Place each data in its own section
  -fdebug-compilation-dir=<value>
                          The compilation directory to embed in the debug info
  -fdebug-default-version=<value>
                          Default DWARF version to use, if a -g option caused DWARF debug info to be produced
  -fdebug-info-for-profiling
                          Emit extra debug info to make sample profile more accurate
  -fdebug-macro           Emit macro debug information
  -fdebug-prefix-map=<old>=<new>
                          For paths in debug info, remap directory <old> to <new>. If multiple options match a path, the last option wins
  -fdebug-ranges-base-address
                          Use DWARF base address selection entries in .debug_ranges
  -fdebug-types-section   Place debug types in their own section (ELF Only)
  -fdeclspec              Allow __declspec as a keyword
  -fdefine-target-os-macros
                          Enable predefined target OS macros
  -fdelayed-template-parsing
                          Parse templated function definitions at the end of the translation unit
  -fdelete-null-pointer-checks
                          Treat usage of null pointers as undefined behavior (default)
  -fdiagnostics-absolute-paths
                          Print absolute paths in diagnostics
  -fdiagnostics-hotness-threshold=<value>
                          Prevent optimization remarks from being output if they do not have at least this profile count. Use 'auto' to apply the threshold from profile summary
  -fdiagnostics-misexpect-tolerance=<value>
                          Prevent misexpect diagnostics from being output if the profile counts are within N% of the expected.
  -fdiagnostics-parseable-fixits
                          Print fix-its in machine parseable form
  -fdiagnostics-print-source-range-info
                          Print source range spans in numeric form
  -fdiagnostics-show-hotness
                          Enable profile hotness information in diagnostic line
  -fdiagnostics-show-note-include-stack
                          Display include stacks for diagnostic notes
  -fdiagnostics-show-option
                          Print option name with mappable diagnostics
  -fdiagnostics-show-template-tree
                          Print a template comparison tree for differing templates
  -fdigraphs              Enable alternative token representations '<:', ':>', '<%', '%>', '%:', '%:%:' (default)
  -fdirect-access-external-data
                          Don't use GOT indirection to reference external data symbols
  -fdiscard-value-names   Discard value names in LLVM IR
  -fdollars-in-identifiers
                          Allow '$' in identifiers
  -fdriver-only           Only run the driver.
  -fdwarf-exceptions      Use DWARF style exceptions
  -feliminate-unused-debug-types
                          Do not emit  debug info for defined but unused types
  -fembed-bitcode-marker  Embed placeholder LLVM IR data as a marker
  -fembed-bitcode=<option>
                          Embed LLVM bitcode
  -fembed-bitcode         Embed LLVM IR bitcode as data
  -fembed-offload-object=<value>
                          Embed Offloading device-side binary into host object file as a section.
  -femit-all-decls        Emit all declarations, even if unused
  -femit-compact-unwind-non-canonical
                          Try emitting Compact-Unwind for non-canonical entries. Maybe overriden by other constraints
  -femit-dwarf-unwind=<value>
                          When to emit DWARF unwind (EH frame) info
  -femulated-tls          Use emutls functions to access thread_local variables
  -fenable-matrix         Enable matrix data type and related builtin functions
  -fexceptions            Enable support for exception handling
  -fexcess-precision=<value>
                          Allows control over excess precision on targets where native support for the precision types is not available. By default, excess precision is used to calculate intermediate results following the rules specified in ISO C99.
  -fexperimental-library  Control whether unstable and experimental library features are enabled. This option enables various library features that are either experimental (also known as TSes), or have been but are not stable yet in the selected Standard Library implementation. It is not recommended to use this option in production code, since neither ABI nor API stability are guaranteed. This is intended to provide a preview of features that will ship in the future for experimentation purposes
  -fexperimental-new-constant-interpreter
                          Enable the experimental new constant interpreter
  -fexperimental-openacc-macro-override <value>
                          Overrides the _OPENACC macro value for experimental testing during OpenACC support development
  -fexperimental-relative-c++-abi-vtables
                          Use the experimental C++ class ABI for classes with virtual tables
  -fexperimental-sanitize-metadata-ignorelist=<value>
                          Disable sanitizer metadata for modules and functions that match the provided special case list
  -fexperimental-sanitize-metadata=<value>
                          Specify the type of metadata to emit for binary analysis sanitizers
  -fexperimental-strict-floating-point
                          Enables the use of non-default rounding modes and non-default exception handling on targets that are not currently ready.
  -fextend-arguments=<value>
                          Controls how scalar integer arguments are extended in calls to unprototyped and varargs functions
  -ffast-math             Allow aggressive, lossy floating-point optimizations
  -ffat-lto-objects       Enable fat LTO object support
  -ffile-compilation-dir=<value>
                          The compilation directory to embed in the debug info and coverage mapping.
  -ffile-prefix-map=<value>
                          remap file source paths in debug info, predefined preprocessor macros and __builtin_FILE(). Implies -ffile-reproducible.
  -ffile-reproducible     Use the target's platform-specific path separator character when expanding the __FILE__ macro
  -ffine-grained-bitfield-accesses
                          Use separate accesses for consecutive bitfield runs with legal widths and alignments.
  -ffinite-loops          Assume all loops are finite.
  -ffinite-math-only      Allow floating-point optimizations that assume arguments and results are not NaNs or +-inf. This defines the __FINITE_MATH_ONLY__ preprocessor macro.
  -ffixed-a0              Reserve the a0 register (M68k only)
  -ffixed-a1              Reserve the a1 register (M68k only)
  -ffixed-a2              Reserve the a2 register (M68k only)
  -ffixed-a3              Reserve the a3 register (M68k only)
  -ffixed-a4              Reserve the a4 register (M68k only)
  -ffixed-a5              Reserve the a5 register (M68k only)
  -ffixed-a6              Reserve the a6 register (M68k only)
  -ffixed-d0              Reserve the d0 register (M68k only)
  -ffixed-d1              Reserve the d1 register (M68k only)
  -ffixed-d2              Reserve the d2 register (M68k only)
  -ffixed-d3              Reserve the d3 register (M68k only)
  -ffixed-d4              Reserve the d4 register (M68k only)
  -ffixed-d5              Reserve the d5 register (M68k only)
  -ffixed-d6              Reserve the d6 register (M68k only)
  -ffixed-d7              Reserve the d7 register (M68k only)
  -ffixed-g1              Reserve the G1 register (SPARC only)
  -ffixed-g2              Reserve the G2 register (SPARC only)
  -ffixed-g3              Reserve the G3 register (SPARC only)
  -ffixed-g4              Reserve the G4 register (SPARC only)
  -ffixed-g5              Reserve the G5 register (SPARC only)
  -ffixed-g6              Reserve the G6 register (SPARC only)
  -ffixed-g7              Reserve the G7 register (SPARC only)
  -ffixed-i0              Reserve the I0 register (SPARC only)
  -ffixed-i1              Reserve the I1 register (SPARC only)
  -ffixed-i2              Reserve the I2 register (SPARC only)
  -ffixed-i3              Reserve the I3 register (SPARC only)
  -ffixed-i4              Reserve the I4 register (SPARC only)
  -ffixed-i5              Reserve the I5 register (SPARC only)
  -ffixed-l0              Reserve the L0 register (SPARC only)
  -ffixed-l1              Reserve the L1 register (SPARC only)
  -ffixed-l2              Reserve the L2 register (SPARC only)
  -ffixed-l3              Reserve the L3 register (SPARC only)
  -ffixed-l4              Reserve the L4 register (SPARC only)
  -ffixed-l5              Reserve the L5 register (SPARC only)
  -ffixed-l6              Reserve the L6 register (SPARC only)
  -ffixed-l7              Reserve the L7 register (SPARC only)
  -ffixed-o0              Reserve the O0 register (SPARC only)
  -ffixed-o1              Reserve the O1 register (SPARC only)
  -ffixed-o2              Reserve the O2 register (SPARC only)
  -ffixed-o3              Reserve the O3 register (SPARC only)
  -ffixed-o4              Reserve the O4 register (SPARC only)
  -ffixed-o5              Reserve the O5 register (SPARC only)
  -ffixed-point           Enable fixed point types
  -ffixed-r19             Reserve register r19 (Hexagon only)
  -ffixed-r9              Reserve the r9 register (ARM only)
  -ffixed-x10             Reserve the x10 register (AArch64/RISC-V only)
  -ffixed-x11             Reserve the x11 register (AArch64/RISC-V only)
  -ffixed-x12             Reserve the x12 register (AArch64/RISC-V only)
  -ffixed-x13             Reserve the x13 register (AArch64/RISC-V only)
  -ffixed-x14             Reserve the x14 register (AArch64/RISC-V only)
  -ffixed-x15             Reserve the x15 register (AArch64/RISC-V only)
  -ffixed-x16             Reserve the x16 register (AArch64/RISC-V only)
  -ffixed-x17             Reserve the x17 register (AArch64/RISC-V only)
  -ffixed-x18             Reserve the x18 register (AArch64/RISC-V only)
  -ffixed-x19             Reserve the x19 register (AArch64/RISC-V only)
  -ffixed-x1              Reserve the x1 register (AArch64/RISC-V only)
  -ffixed-x20             Reserve the x20 register (AArch64/RISC-V only)
  -ffixed-x21             Reserve the x21 register (AArch64/RISC-V only)
  -ffixed-x22             Reserve the x22 register (AArch64/RISC-V only)
  -ffixed-x23             Reserve the x23 register (AArch64/RISC-V only)
  -ffixed-x24             Reserve the x24 register (AArch64/RISC-V only)
  -ffixed-x25             Reserve the x25 register (AArch64/RISC-V only)
  -ffixed-x26             Reserve the x26 register (AArch64/RISC-V only)
  -ffixed-x27             Reserve the x27 register (AArch64/RISC-V only)
  -ffixed-x28             Reserve the x28 register (AArch64/RISC-V only)
  -ffixed-x29             Reserve the x29 register (AArch64/RISC-V only)
  -ffixed-x2              Reserve the x2 register (AArch64/RISC-V only)
  -ffixed-x30             Reserve the x30 register (AArch64/RISC-V only)
  -ffixed-x31             Reserve the x31 register (AArch64/RISC-V only)
  -ffixed-x3              Reserve the x3 register (AArch64/RISC-V only)
  -ffixed-x4              Reserve the x4 register (AArch64/RISC-V only)
  -ffixed-x5              Reserve the x5 register (AArch64/RISC-V only)
  -ffixed-x6              Reserve the x6 register (AArch64/RISC-V only)
  -ffixed-x7              Reserve the x7 register (AArch64/RISC-V only)
  -ffixed-x8              Reserve the x8 register (AArch64/RISC-V only)
  -ffixed-x9              Reserve the x9 register (AArch64/RISC-V only)
  -fforce-check-cxx20-modules-input-files
                          Check the input source files from C++20 modules explicitly
  -fforce-dwarf-frame     Always emit a debug frame section
  -fforce-emit-vtables    Emits more virtual tables to improve devirtualization
  -fforce-enable-int128   Enable support for int128_t type
  -ffp-contract=<value>   Form fused FP ops (e.g. FMAs)
  -ffp-eval-method=<value>
                          Specifies the evaluation method to use for floating-point arithmetic.
  -ffp-exception-behavior=<value>
                          Specifies the exception behavior of floating-point operations.
  -ffp-model=<value>      Controls the semantics of floating-point calculations.
  -ffreestanding          Assert that the compilation takes place in a freestanding environment
  -ffuchsia-api-level=<value>
                          Set Fuchsia API level
  -ffunction-sections     Place each function in its own section
  -fglobal-isel           Enables the global instruction selector
  -fgnu-keywords          Allow GNU-extension keywords regardless of language standard
  -fgnu-runtime           Generate output compatible with the standard GNU Objective-C runtime
  -fgnu89-inline          Use the gnu89 inline semantics
  -fgnuc-version=<value>  Sets various macros to claim compatibility with the given GCC version (default is 4.2.1)
  -fgpu-allow-device-init Allow device side init function in HIP (experimental)
  -fgpu-approx-transcendentals
                          Use approximate transcendental functions
  -fgpu-default-stream=<value>
                          Specify default stream. The default value is 'legacy'. (CUDA/HIP only)
  -fgpu-defer-diag        Defer host/device related diagnostic messages for CUDA/HIP
  -fgpu-exclude-wrong-side-overloads
                          Always exclude wrong side overloads in overloading resolution for CUDA/HIP
  -fgpu-flush-denormals-to-zero
                          Flush denormal floating point values to zero in CUDA/HIP device mode.
  -fgpu-inline-threshold=<value>
                          Inline threshold for device compilation for CUDA/HIP
  -fgpu-rdc               Generate relocatable device code, also known as separate compilation mode
  -fgpu-sanitize          Enable sanitizer for supported offloading devices
  -fhip-emit-relocatable  Compile HIP source to relocatable
  -fhip-fp32-correctly-rounded-divide-sqrt
                          Specify that single precision floating-point divide and sqrt used in the program source are correctly rounded (HIP device compilation only)
  -fhip-kernel-arg-name   Specify that kernel argument names are preserved (HIP only)
  -fhip-new-launch-api    Use new kernel launching API for HIP
  -fhonor-infinities      Specify that floating-point optimizations are not allowed that assume arguments and results are not +-inf.
  -fhonor-nans            Specify that floating-point optimizations are not allowed that assume arguments and results are not NANs.
  -fignore-exceptions     Enable support for ignoring exception handling constructs
  -fimplicit-module-maps  Implicitly search the file system for module map files.
  -fincremental-extensions
                          Enable incremental processing extensions such as processingstatements on the global scope.
  -finline-functions      Inline suitable functions
  -finline-hint-functions Inline functions which are (explicitly or implicitly) marked inline
  -finline-max-stacksize=<value>
                          Suppress inlining of functions whose stack size exceeds the given value
  -finput-charset=<value> Specify the default character set for source files
  -finstrument-function-entry-bare
                          Instrument function entry only, after inlining, without arguments to the instrumentation call
  -finstrument-functions-after-inlining
                          Like -finstrument-functions, but insert the calls after inlining
  -finstrument-functions  Generate calls to instrument function entry and exit
  -fintegrated-as         Enable the integrated assembler
  -fintegrated-cc1        Run cc1 in-process
  -fintegrated-objemitter Use internal machine object code emitter.
  -fjmc                   Enable just-my-code debugging
  -fjump-tables           Use jump tables for lowering switches
  -fkeep-persistent-storage-variables
                          Enable keeping all variables that have a persistent storage duration, including global, static and thread-local variables, to guarantee that they can be directly addressed
  -fkeep-static-consts    Keep static const variables even if unused
  -fkeep-system-includes  Instead of expanding system headers when emitting preprocessor output, preserve the #include directive. Useful when producing preprocessed output for test case reduction. May produce incorrect output if preprocessor symbols that control the included content (e.g. _XOPEN_SOURCE) are defined in the including source file. The portability of the resulting source to other compilation environments is not guaranteed.

                          Only valid with -E.
  -flax-vector-conversions=<value>
                          Enable implicit vector bit-casts
  -flto-jobs=<value>      Controls the backend parallelism of -flto=thin (default of 0 means the number of threads will be derived from the number of CPUs detected)
  -flto=auto              Enable LTO in 'full' mode
  -flto=jobserver         Enable LTO in 'full' mode
  -flto=<value>           Set LTO mode
  -flto                   Enable LTO in 'full' mode
  -fmacro-backtrace-limit=<value>
                          Set the maximum number of entries to print in a macro expansion backtrace (0 = no limit)
  -fmacro-prefix-map=<value>
                          remap file source paths in predefined preprocessor macros and __builtin_FILE(). Implies -ffile-reproducible.
  -fmath-errno            Require math functions to indicate errors by setting errno
  -fmax-tokens=<value>    Max total number of preprocessed tokens for -Wmax-tokens.
  -fmax-type-align=<value>
                          Specify the maximum alignment to enforce on pointers lacking an explicit alignment
  -fmemory-profile-use=<pathname>
                          Use memory profile for profile-guided memory optimization
  -fmemory-profile=<directory>
                          Enable heap memory profiling and dump results into <directory>
  -fmemory-profile        Enable heap memory profiling
  -fmerge-all-constants   Allow merging of constants
  -fmessage-length=<value>
                          Format message diagnostics so that they fit within N columns
  -fminimize-whitespace   Ignore the whitespace from the input file when emitting preprocessor output. It will only contain whitespace when necessary, e.g. to keep two minus signs from merging into to an increment operator. Useful with the -P option to normalize whitespace such that two files with only formatting changes are equal.

                          Only valid with -E on C-like inputs and incompatible with -traditional-cpp.
  -fmodule-file=[<name>=]<file>
                          Specify the mapping of module name to precompiled module file, or load a module file if name is omitted.
  -fmodule-header=<kind>  Build a C++20 Header Unit from a header that should be found in the user (fmodule-header=user) or system (fmodule-header=system) search path.
  -fmodule-header         Build a C++20 Header Unit from a header
  -fmodule-map-file=<file>
                          Load this module map file
  -fmodule-name=<name>    Specify the name of the module to build
  -fmodule-output=<value> Save intermediate module file results when compiling a standard C++ module unit.
  -fmodule-output         Save intermediate module file results when compiling a standard C++ module unit.
  -fmodules-cache-path=<directory>
                          Specify the module cache path
  -fmodules-decluse       Require declaration of modules used within a module
  -fmodules-disable-diagnostic-validation
                          Disable validation of the diagnostic options when loading the module
  -fmodules-ignore-macro=<value>
                          Ignore the definition of the given macro when building and loading modules
  -fmodules-prune-after=<seconds>
                          Specify the interval (in seconds) after which a module file will be considered unused
  -fmodules-prune-interval=<seconds>
                          Specify the interval (in seconds) between attempts to prune the module cache
  -fmodules-search-all    Search even non-imported modules to resolve references
  -fmodules-strict-decluse
                          Like -fmodules-decluse but requires all headers to be in modules
  -fmodules-user-build-path <directory>
                          Specify the module user build path
  -fmodules-validate-input-files-content
                          Validate PCM input files based on content if mtime differs
  -fmodules-validate-once-per-build-session
                          Don't verify input files for the modules if the module has been successfully validated or loaded during this build session
  -fmodules-validate-system-headers
                          Validate the system headers that a module depends on when loading the module
  -fmodules               Enable the 'modules' language feature
  -fms-compatibility-version=<value>
                          Dot-separated value representing the Microsoft compiler version number to report in _MSC_VER (0 = don't define it (default))
  -fms-compatibility      Enable full Microsoft Visual C++ compatibility
  -fms-extensions         Accept some non-standard constructs supported by the Microsoft compiler
  -fms-hotpatch           Ensure that all functions can be hotpatched at runtime
  -fms-runtime-lib=<value>
                          Select Windows run-time library
  -fms-volatile           Volatile loads and stores have acquire and release semantics
  -fmsc-version=<value>   Microsoft compiler version number to report in _MSC_VER (0 = don't define it (default))
  -fnew-alignment=<align> Specifies the largest alignment guaranteed by '::operator new(size_t)'
  -fnew-infallible        Enable treating throwing global C++ operator new as always returning valid memory (annotates with __attribute__((returns_nonnull)) and throw()). This is detectable in source.
  -fno-aapcs-bitfield-width
                          Do not follow the AAPCS standard requirement stating that volatile bit-field width is dictated by the field container type. (ARM only).
  -fno-access-control     Disable C++ access control
  -fno-addrsig            Don't emit an address-significance table
  -fno-apinotes-modules   Disablemodule-based external API notes support
  -fno-apinotes           Disableexternal API notes support
  -fno-assume-sane-operator-new
                          Don't assume that C++'s global operator new can't alias any pointer
  -fno-assume-unique-vtables
                          Disable optimizations based on vtable pointer identity
  -fno-assumptions        Disable codegen and compile-time checks for C++23's [[assume]] attribute
  -fno-auto-import        MinGW specific. Disable support for automatic dllimport in code generation and linking
  -fno-autolink           Disable generation of linker directives for automatic library linking
  -fno-builtin-<value>    Disable implicit builtin knowledge of a specific function
  -fno-builtin            Disable implicit builtin knowledge of functions
  -fno-c++-static-destructors
                          Disable C++ static destructor registration
  -fno-char8_t            Disable C++ builtin type char8_t
  -fno-color-diagnostics  Disable colors in diagnostics
  -fno-common             Compile common globals like normal definitions
  -fno-complete-member-pointers
                          Do not require member pointer base types to be complete if they would be significant under the Microsoft ABI
  -fno-constant-cfstrings Disable creation of CodeFoundation-type constant strings
  -fno-convergent-functions
                          Assume all functions may be convergent.
  -fno-coverage-mapping   Disable code coverage analysis
  -fno-coverage-mcdc      Disable MC/DC coverage criteria
  -fno-crash-diagnostics  Disable auto-generation of preprocessed source files and a script for reproduction during a clang crash
  -fno-cx-fortran-rules   Range reduction is disabled for complex arithmetic operations
  -fno-cx-limited-range   Basic algebraic expansions of complex arithmetic operations involving are disabled.
  -fno-cxx-modules        Disable modules for C++
  -fno-debug-macro        Do not emit macro debug information
  -fno-declspec           Disallow __declspec as a keyword
  -fno-define-target-os-macros
                          Disable predefined target OS macros
  -fno-delayed-template-parsing
                          Disable delayed template parsing
  -fno-delete-null-pointer-checks
                          Do not treat usage of null pointers as undefined behavior
  -fno-diagnostics-fixit-info
                          Do not include fixit information in diagnostics
  -fno-diagnostics-show-line-numbers
                          Show line numbers in diagnostic code snippets
  -fno-digraphs           Disallow alternative token representations '<:', ':>', '<%', '%>', '%:', '%:%:'
  -fno-direct-access-external-data
                          Use GOT indirection to reference external data symbols
  -fno-discard-value-names
                          Do not discard value names in LLVM IR
  -fno-dollars-in-identifiers
                          Disallow '$' in identifiers
  -fno-elide-constructors Disable C++ copy constructor elision
  -fno-elide-type         Do not elide types when printing diagnostics
  -fno-eliminate-unused-debug-types
                          Emit  debug info for defined but unused types
  -fno-exceptions         Disable support for exception handling
  -fno-experimental-relative-c++-abi-vtables
                          Do not use the experimental C++ class ABI for classes with virtual tables
  -fno-experimental-sanitize-metadata=<value>
                          Disable emitting metadata for binary analysis sanitizers
  -fno-fat-lto-objects    Disable fat LTO object support
  -fno-file-reproducible  Use the host's platform-specific path separator character when expanding the __FILE__ macro
  -fno-fine-grained-bitfield-accesses
                          Use large-integer access for consecutive bitfield runs.
  -fno-finite-loops       Do not assume that any loop is finite.
  -fno-fixed-point        Disable fixed point types
  -fno-force-enable-int128
                          Disable support for int128_t type
  -fno-global-isel        Disables the global instruction selector
  -fno-gnu-inline-asm     Disable GNU style inline asm
  -fno-gpu-allow-device-init
                          Don't allow device side init function in HIP (experimental)
  -fno-gpu-approx-transcendentals
                          Don't use approximate transcendental functions
  -fno-gpu-defer-diag     Don't defer host/device related diagnostic messages for CUDA/HIP
  -fno-gpu-exclude-wrong-side-overloads
                          Exclude wrong side overloads only if there are same side overloads in overloading resolution for CUDA/HIP
  -fno-hip-emit-relocatable
                          Do not override toolchain to compile HIP source to relocatable
  -fno-hip-fp32-correctly-rounded-divide-sqrt
                          Don't specify that single precision floating-point divide and sqrt used in the program source are correctly rounded (HIP device compilation only)
  -fno-hip-kernel-arg-name
                          Don't specify that kernel argument names are preserved (HIP only)
  -fno-hip-new-launch-api Don't use new kernel launching API for HIP
  -fno-integrated-as      Disable the integrated assembler
  -fno-integrated-cc1     Spawn a separate process for each cc1
  -fno-integrated-objemitter
                          Use external machine object code emitter.
  -fno-jump-tables        Do not use jump tables for lowering switches
  -fno-keep-persistent-storage-variables
                          Disable keeping all variables that have a persistent storage duration, including global, static and thread-local variables, to guarantee that they can be directly addressed
  -fno-keep-static-consts Don't keep static const variables even if unused
  -fno-knr-functions      Disable support for K&R C function declarations
  -fno-lto                Disable LTO mode (default)
  -fno-memory-profile     Disable heap memory profiling
  -fno-merge-all-constants
                          Disallow merging of constants
  -fno-modules-check-relocated<value>
                          Skip checks for relocated modules when loading PCM files
  -fno-modules-validate-textual-header-includes
                          Do not enforce -fmodules-decluse and private header restrictions for textual headers. This flag will be removed in a future Clang release.
  -fno-new-infallible     Disable treating throwing global C++ operator new as always returning valid memory (annotates with __attribute__((returns_nonnull)) and throw()). This is detectable in source.
  -fno-objc-avoid-heapify-local-blocks
                          Don't try to avoid heapifying local blocks
  -fno-objc-infer-related-result-type
                          do not infer Objective-C related result type based on method family
  -fno-offload-lto        Disable LTO mode (default) for offload compilation
  -fno-offload-uniform-block
                          Don't assume that kernels are launched with uniform block sizes (default true for CUDA/HIP and false otherwise)
  -fno-openmp-extensions  Disable all Clang extensions for OpenMP directives and clauses
  -fno-openmp-new-driver  Don't use the new driver for OpenMP offloading.
  -fno-operator-names     Do not treat C++ operator name keywords as synonyms for operators
  -fno-optimize-sibling-calls
                          Disable tail call optimization, keeping the call stack accurate
  -fno-pch-codegen        Do not generate code for uses of this PCH that assumes an explicit object file will be built for the PCH
  -fno-pch-debuginfo      Do not generate debug info for types in an object file built from this PCH and do not generate them elsewhere
  -fno-plt                Use GOT indirection instead of PLT to make external function calls (x86 only)
  -fno-preserve-as-comments
                          Do not preserve comments in inline assembly
  -fno-profile-generate   Disable generation of profile instrumentation.
  -fno-profile-instr-generate
                          Disable generation of profile instrumentation.
  -fno-profile-instr-use  Disable using instrumentation data for profile-guided optimization
  -fno-pseudo-probe-for-profiling
                          Do not emit pseudo probes for sample profiling
  -fno-register-global-dtors-with-atexit
                          Don't use atexit or __cxa_atexit to register global destructors
  -fno-rtlib-add-rpath    Do not add -rpath with architecture-specific resource directory to the linker flags. When --hip-link is specified, do not add -rpath with HIP runtime library directory to the linker flags
  -fno-rtti-data          Disable generation of RTTI data
  -fno-rtti               Disable generation of rtti information
  -fno-sanitize-address-globals-dead-stripping
                          Disable linker dead stripping of globals in AddressSanitizer
  -fno-sanitize-address-outline-instrumentation
                          Use default code inlining logic for the address sanitizer
  -fno-sanitize-address-poison-custom-array-cookie
                          Disable poisoning array cookies when using custom operator new[] in AddressSanitizer
  -fno-sanitize-address-use-after-scope
                          Disable use-after-scope detection in AddressSanitizer
  -fno-sanitize-address-use-odr-indicator
                          Disable ODR indicator globals
  -fno-sanitize-cfi-canonical-jump-tables
                          Do not make the jump table addresses canonical in the symbol table
  -fno-sanitize-cfi-cross-dso
                          Disable control flow integrity (CFI) checks for cross-DSO calls.
  -fno-sanitize-coverage=<value>
                          Disable features of coverage instrumentation for Sanitizers
  -fno-sanitize-hwaddress-experimental-aliasing
                          Disable aliasing mode in HWAddressSanitizer
  -fno-sanitize-ignorelist
                          Don't use ignorelist file for sanitizers
  -fno-sanitize-memory-param-retval
                          Disable detection of uninitialized parameters and return values
  -fno-sanitize-memory-track-origins
                          Disable origins tracking in MemorySanitizer
  -fno-sanitize-memory-use-after-dtor
                          Disable use-after-destroy detection in MemorySanitizer
  -fno-sanitize-recover=<value>
                          Disable recovery for specified sanitizers
  -fno-sanitize-stable-abi
                          Conventional ABI instrumentation for sanitizer runtime. Default: Conventional
  -fno-sanitize-stats     Disable sanitizer statistics gathering.
  -fno-sanitize-thread-atomics
                          Disable atomic operations instrumentation in ThreadSanitizer
  -fno-sanitize-thread-func-entry-exit
                          Disable function entry/exit instrumentation in ThreadSanitizer
  -fno-sanitize-thread-memory-access
                          Disable memory access instrumentation in ThreadSanitizer
  -fno-sanitize-trap=<value>
                          Disable trapping for specified sanitizers
  -fno-sanitize-trap      Disable trapping for all sanitizers
  -fno-short-wchar        Force wchar_t to be an unsigned int
  -fno-show-column        Do not include column number on diagnostics
  -fno-show-source-location
                          Do not include source location information with diagnostics
  -fno-signed-char        char is unsigned
  -fno-signed-zeros       Allow optimizations that ignore the sign of floating point zeros
  -fno-skip-odr-check-in-gmf
                          Perform ODR checks for decls in the global module fragment.
  -fno-spell-checking     Disable spell-checking
  -fno-split-machine-functions
                          Disable late function splitting using profile information (x86 ELF)
  -fno-split-stack        Wouldn't use segmented stack
  -fno-stack-clash-protection
                          Disable stack clash protection
  -fno-stack-protector    Disable the use of stack protectors
  -fno-standalone-debug   Limit debug information produced to reduce size of debug binary
  -fno-strict-aliasing    Disable optimizations based on strict aliasing rules
  -fno-strict-float-cast-overflow
                          Relax language rules and try to match the behavior of the target's native float-to-int conversion instructions
  -fno-strict-return      Don't treat control flow paths that fall off the end of a non-void function as unreachable
  -fno-sycl               Disables SYCL kernels compilation for device
  -fno-temp-file          Directly create compilation output files. This may lead to incorrect incremental builds if the compiler crashes
  -fno-threadsafe-statics Do not emit code to make initialization of local statics thread safe
  -fno-trigraphs          Do not process trigraph sequences
  -fno-unified-lto        Use distinct LTO pipelines
  -fno-unique-section-names
                          Don't use unique names for text and data sections
  -fno-unroll-loops       Turn off loop unroller
  -fno-use-cxa-atexit     Don't use __cxa_atexit for calling destructors
  -fno-use-init-array     Use .ctors/.dtors instead of .init_array/.fini_array
  -fno-verify-intermediate-code
                          Disable verification of LLVM IR
  -fno-visibility-inlines-hidden-static-local-var
                          Disables -fvisibility-inlines-hidden-static-local-var (this is the default on non-darwin targets)
  -fno-xray-function-index
                          Omit function index section at the expense of single-function patching performance
  -fno-zero-initialized-in-bss
                          Don't place zero initialized data in BSS
  -fobjc-arc-exceptions   Use EH-safe code when synthesizing retains and releases in -fobjc-arc
  -fobjc-arc              Synthesize retain and release calls for Objective-C pointers
  -fobjc-avoid-heapify-local-blocks
                          Try to avoid heapifying local blocks
  -fobjc-disable-direct-methods-for-testing
                          Ignore attribute objc_direct so that direct methods can be tested
  -fobjc-encode-cxx-class-template-spec
                          Fully encode c++ class template specialization
  -fobjc-exceptions       Enable Objective-C exceptions
  -fobjc-runtime=<value>  Specify the target Objective-C runtime kind and version
  -fobjc-weak             Enable ARC-style weak references in Objective-C
  -foffload-implicit-host-device-templates
                          Template functions or specializations without host, device and global attributes have implicit host device attributes (CUDA/HIP only)
  -foffload-lto=<value>   Set LTO mode for offload compilation
  -foffload-lto           Enable LTO in 'full' mode for offload compilation
  -foffload-uniform-block Assume that kernels are launched with uniform block sizes (default true for CUDA/HIP and false otherwise)
  -fomit-frame-pointer    Omit the frame pointer from functions that don't need it. Some stack unwinding cases, such as profilers and sanitizers, may prefer specifying -fno-omit-frame-pointer. On many targets, -O1 and higher omit the frame pointer by default. -m[no-]omit-leaf-frame-pointer takes precedence for leaf functions
  -fopenacc               Enable OpenACC
  -fopenmp-assume-no-nested-parallelism
                          Assert no nested parallel regions in the GPU
  -fopenmp-assume-no-thread-state
                          Assert no thread in a parallel region modifies an ICV
  -fopenmp-enable-irbuilder
                          Use the experimental OpenMP-IR-Builder codegen path.
  -fopenmp-extensions     Enable all Clang extensions for OpenMP directives and clauses
  -fopenmp-force-usm      Force behvaior as if the user specified pragma omp requires unified_shared_memory.
  -fopenmp-new-driver     Use the new driver for OpenMP offloading.
  -fopenmp-offload-mandatory
                          Do not create a host fallback if offloading to the device fails.
  -fopenmp-simd           Emit OpenMP code only for SIMD-based constructs.
  -fopenmp-target-debug   Enable debugging in the OpenMP offloading device RTL
  -fopenmp-target-jit     Emit code that can be JIT compiled for OpenMP offloading. Implies -foffload-lto=full
  -fopenmp-targets=<value>
                          Specify comma-separated list of triples OpenMP offloading targets to be supported
  -fopenmp-version=<value>
                          Set OpenMP version (e.g. 45 for OpenMP 4.5, 51 for OpenMP 5.1). Default value is 51 for Clang
  -fopenmp                Parse OpenMP pragmas and generate parallel code.
  -foperator-arrow-depth=<value>
                          Maximum number of 'operator->'s to call for a member access
  -foptimization-record-file=<file>
                          Specify the output name of the file containing the optimization remarks. Implies -fsave-optimization-record. On Darwin platforms, this cannot be used with multiple -arch <arch> options.
  -foptimization-record-passes=<regex>
                          Only include passes which match a specified regular expression in the generated optimization record (by default, include all passes)
  -forder-file-instrumentation
                          Generate instrumented code to collect order file into default.profraw file (overridden by '=' form of option or LLVM_PROFILE_FILE env var)
  -fpack-struct=<value>   Specify the default maximum struct packing alignment
  -fpascal-strings        Recognize and construct Pascal-style string literals
  -fpass-plugin=<dsopath> Load pass plugin from a dynamic shared object file (only with new pass manager).
  -fpatchable-function-entry=<N,M>
                          Generate M NOPs before function entry and N-M NOPs after function entry
  -fpcc-struct-return     Override the default ABI to return all structs on the stack
  -fpch-codegen           Generate code for uses of this PCH that assumes an explicit object file will be built for the PCH
  -fpch-debuginfo         Generate debug info for types in an object file built from this PCH and do not generate them elsewhere
  -fpch-instantiate-templates
                          Instantiate templates already while building a PCH
  -fpch-validate-input-files-content
                          Validate PCH input files based on content if mtime differs
  -fplugin-arg-<name>-<arg>
                          Pass <arg> to plugin <name>
  -fplugin=<dsopath>      Load the named plugin (dynamic shared object)
  -fprebuilt-implicit-modules
                          Look up implicit modules in the prebuilt module path
  -fprebuilt-module-path=<directory>
                          Specify the prebuilt module path
  -fproc-stat-report=<value>
                          Save subprocess statistics to the given file
  -fproc-stat-report<value>
                          Print subprocess statistics
  -fprofile-arcs          Instrument code to produce gcov data files (*.gcda)
  -fprofile-exclude-files=<value>
                          Instrument only functions from files where names don't match all the regexes separated by a semi-colon
  -fprofile-filter-files=<value>
                          Instrument only functions from files where names match any regex separated by a semi-colon
  -fprofile-function-groups=<N>
                          Partition functions into N groups and select only functions in group i to be instrumented using -fprofile-selected-function-group
  -fprofile-generate=<directory>
                          Generate instrumented code to collect execution counts into <directory>/default.profraw (overridden by LLVM_PROFILE_FILE env var)
  -fprofile-generate      Generate instrumented code to collect execution counts into default.profraw (overridden by LLVM_PROFILE_FILE env var)
  -fprofile-instr-generate=<file>
                          Generate instrumented code to collect execution counts into <file> (overridden by LLVM_PROFILE_FILE env var)
  -fprofile-instr-generate
                          Generate instrumented code to collect execution counts into default.profraw file (overridden by '=' form of option or LLVM_PROFILE_FILE env var)
  -fprofile-instr-use=<value>
                          Use instrumentation data for profile-guided optimization
  -fprofile-list=<value>  Filename defining the list of functions/files to instrument. The file uses the sanitizer special case list format.
  -fprofile-remapping-file=<file>
                          Use the remappings described in <file> to match the profile data against names in the program
  -fprofile-sample-accurate
                          Specifies that the sample profile is accurate
  -fprofile-sample-use=<value>
                          Enable sample-based profile guided optimizations
  -fprofile-selected-function-group=<i>
                          Partition functions into N groups using -fprofile-function-groups and select only functions in group i to be instrumented. The valid range is 0 to N-1 inclusive
  -fprofile-update=<method>
                          Set update method of profile counters
  -fprofile-use=<pathname>
                          Use instrumentation data for profile-guided optimization. If pathname is a directory, it reads from <pathname>/default.profdata. Otherwise, it reads from file <pathname>.
  -fprotect-parens        Determines whether the optimizer honors parentheses when floating-point expressions are evaluated
  -fpseudo-probe-for-profiling
                          Emit pseudo probes for sample profiling
  -fptrauth-intrinsics    Enable pointer authentication intrinsics
  -frandomize-layout-seed-file=<file>
                          File holding the seed used by the randomize structure layout feature
  -frandomize-layout-seed=<seed>
                          The seed used by the randomize structure layout feature
  -freciprocal-math       Allow division operations to be reassociated
  -freg-struct-return     Override the default ABI to return small structs in registers
  -fregister-global-dtors-with-atexit
                          Use atexit or __cxa_atexit to register global destructors
  -frelaxed-template-template-args
                          Enable C++17 relaxed template template argument matching
  -fropi                  Generate read-only position independent code (ARM only)
  -frtlib-add-rpath       Add -rpath with architecture-specific resource directory to the linker flags. When --hip-link is specified, also add -rpath with HIP runtime library directory to the linker flags
  -frwpi                  Generate read-write position independent code (ARM only)
  -fsafe-buffer-usage-suggestions
                          Display suggestions to update code associated with -Wunsafe-buffer-usage warnings
  -fsample-profile-use-profi
                          Use profi to infer block and edge counts
  -fsanitize-address-destructor=<value>
                          Set the kind of module destructors emitted by AddressSanitizer instrumentation. These destructors are emitted to unregister instrumented global variables when code is unloaded (e.g. via `dlclose()`).
  -fsanitize-address-field-padding=<value>
                          Level of field padding for AddressSanitizer
  -fsanitize-address-globals-dead-stripping
                          Enable linker dead stripping of globals in AddressSanitizer
  -fsanitize-address-outline-instrumentation
                          Always generate function calls for address sanitizer instrumentation
  -fsanitize-address-poison-custom-array-cookie
                          Enable poisoning array cookies when using custom operator new[] in AddressSanitizer
  -fsanitize-address-use-after-return=<mode>
                          Select the mode of detecting stack use-after-return in AddressSanitizer
  -fsanitize-address-use-after-scope
                          Enable use-after-scope detection in AddressSanitizer
  -fsanitize-address-use-odr-indicator
                          Enable ODR indicator globals to avoid false ODR violation reports in partially sanitized programs at the cost of an increase in binary size
  -fsanitize-blacklist=<value>
                          Alias for -fsanitize-ignorelist=
  -fsanitize-cfi-canonical-jump-tables
                          Make the jump table addresses canonical in the symbol table
  -fsanitize-cfi-cross-dso
                          Enable control flow integrity (CFI) checks for cross-DSO calls.
  -fsanitize-cfi-icall-experimental-normalize-integers
                          Normalize integers in CFI indirect call type signature checks
  -fsanitize-cfi-icall-generalize-pointers
                          Generalize pointers in CFI indirect call type signature checks
  -fsanitize-coverage-allowlist=<value>
                          Restrict sanitizer coverage instrumentation exclusively to modules and functions that match the provided special case list, except the blocked ones
  -fsanitize-coverage-ignorelist=<value>
                          Disable sanitizer coverage instrumentation for modules and functions that match the provided special case list, even the allowed ones
  -fsanitize-coverage=<value>
                          Specify the type of coverage instrumentation for Sanitizers
  -fsanitize-hwaddress-abi=<value>
                          Select the HWAddressSanitizer ABI to target (interceptor or platform, default interceptor). This option is currently unused.
  -fsanitize-hwaddress-experimental-aliasing
                          Enable aliasing mode in HWAddressSanitizer
  -fsanitize-ignorelist=<value>
                          Path to ignorelist file for sanitizers
  -fsanitize-memory-param-retval
                          Enable detection of uninitialized parameters and return values
  -fsanitize-memory-track-origins=<value>
                          Enable origins tracking in MemorySanitizer
  -fsanitize-memory-track-origins
                          Enable origins tracking in MemorySanitizer
  -fsanitize-memory-use-after-dtor
                          Enable use-after-destroy detection in MemorySanitizer
  -fsanitize-memtag-mode=<value>
                          Set default MTE mode to 'sync' (default) or 'async'
  -fsanitize-recover=<value>
                          Enable recovery for specified sanitizers
  -fsanitize-stable-abi   Stable  ABI instrumentation for sanitizer runtime. Default: Conventional
  -fsanitize-stats        Enable sanitizer statistics gathering.
  -fsanitize-system-ignorelist=<value>
                          Path to system ignorelist file for sanitizers
  -fsanitize-thread-atomics
                          Enable atomic operations instrumentation in ThreadSanitizer (default)
  -fsanitize-thread-func-entry-exit
                          Enable function entry/exit instrumentation in ThreadSanitizer (default)
  -fsanitize-thread-memory-access
                          Enable memory access instrumentation in ThreadSanitizer (default)
  -fsanitize-trap=<value> Enable trapping for specified sanitizers
  -fsanitize-trap         Enable trapping for all sanitizers
  -fsanitize-undefined-strip-path-components=<number>
                          Strip (or keep only, if negative) a given number of path components when emitting check metadata.
  -fsanitize=<check>      Turn on runtime checks for various forms of undefined or suspicious behavior. See user manual for available checks
  -fsave-optimization-record=<format>
                          Generate an optimization record file in a specific format
  -fsave-optimization-record
                          Generate a YAML optimization record file
  -fseh-exceptions        Use SEH style exceptions
  -fshort-enums           Allocate to an enum type only as many bytes as it needs for the declared range of possible values
  -fshort-wchar           Force wchar_t to be a short unsigned int
  -fshow-overloads=<value>
                          Which overload candidates to show when overload resolution fails. Defaults to 'all'
  -fshow-skipped-includes Show skipped includes in -H output.
  -fsigned-char           char is signed
  -fsized-deallocation    Enable C++14 sized global deallocation functions
  -fsjlj-exceptions       Use SjLj style exceptions
  -fskip-odr-check-in-gmf Skip ODR checks for decls in the global module fragment.
  -fslp-vectorize         Enable the superword-level parallelism vectorization passes
  -fspell-checking-limit=<value>
                          Set the maximum number of times to perform spell checking on unrecognized identifiers (0 = no limit)
  -fsplit-dwarf-inlining  Provide minimal debug info in the object/executable to facilitate online symbolication/stack traces in the absence of .dwo/.dwp files when using Split DWARF
  -fsplit-lto-unit        Enables splitting of the LTO unit
  -fsplit-machine-functions
                          Enable late function splitting using profile information (x86 ELF)
  -fsplit-stack           Use segmented stack
  -fstack-clash-protection
                          Enable stack clash protection
  -fstack-protector-all   Enable stack protectors for all functions
  -fstack-protector-strong
                          Enable stack protectors for some functions vulnerable to stack smashing. Compared to -fstack-protector, this uses a stronger heuristic that includes functions containing arrays of any size (and any type), as well as any calls to alloca or the taking of an address from a local variable
  -fstack-protector       Enable stack protectors for some functions vulnerable to stack smashing. This uses a loose heuristic which considers functions vulnerable if they contain a char (or 8bit integer) array or constant sized calls to alloca , which are of greater size than ssp-buffer-size (default: 8 bytes). All variable sized calls to alloca are considered vulnerable. A function with a stack protector has a guard value added to the stack frame that is checked on function exit. The guard value must be positioned in the stack frame such that a buffer overflow from a vulnerable variable will overwrite the guard value before overwriting the function's return address. The reference stack guard value is stored in a global variable.
  -fstack-size-section    Emit section containing metadata on function stack sizes
  -fstack-usage           Emit .su file containing information on function stack sizes
  -fstandalone-debug      Emit full debug info for all types used by the program
  -fstrict-aliasing       Enable optimizations based on strict aliasing rules
  -fstrict-enums          Enable optimizations based on the strict definition of an enum's value range
  -fstrict-flex-arrays=<n>
                          Enable optimizations based on the strict definition of flexible arrays
  -fstrict-float-cast-overflow
                          Assume that overflowing float-to-int casts are undefined (default)
  -fstrict-vtable-pointers
                          Enable optimizations based on the strict rules for overwriting polymorphic C++ objects
  -fswift-async-fp=<option>
                          Control emission of Swift async extended frame info
  -fsycl                  Enables SYCL kernels compilation for device
  -fsyntax-only           Run the preprocessor, parser and semantic analysis stages
  -fsystem-module         Build this module as a system module. Only used with -emit-module
  -ftemplate-backtrace-limit=<value>
                          Set the maximum number of entries to print in a template instantiation backtrace (0 = no limit)
  -ftemplate-depth=<value>
                          Set the maximum depth of recursive template instantiation
  -ftest-coverage         Produce gcov notes files (*.gcno)
  -fthin-link-bitcode=<value>
                          Write minimized bitcode to <file> for the ThinLTO thin link only
  -fthinlto-index=<value> Perform ThinLTO importing using provided function summary index
  -ftime-report=<value>   (For new pass manager) 'per-pass': one report for each pass; 'per-pass-run': one report for each pass invocation
  -ftime-trace-granularity=<value>
                          Minimum time granularity (in microseconds) traced by time profiler
  -ftime-trace=<value>    Similar to -ftime-trace. Specify the JSON file or a directory which will contain the JSON file
  -ftime-trace            Turn on time profiler. Generates JSON file based on output filename.
  -ftrap-function=<value> Issue call to specified function rather than a trap instruction
  -ftrapv-handler=<function name>
                          Specify the function to be called on overflow
  -ftrapv                 Trap on integer overflow
  -ftrigraphs             Process trigraph sequences
  -ftrivial-auto-var-init-max-size=<value>
                          Stop initializing trivial automatic stack variables if var size exceeds the specified number of instances (in bytes)
  -ftrivial-auto-var-init-stop-after=<value>
                          Stop initializing trivial automatic stack variables after the specified number of instances
  -ftrivial-auto-var-init=<value>
                          Initialize trivial automatic stack variables. Defaults to 'uninitialized'
  -funified-lto           Use the unified LTO pipeline
  -funique-basic-block-section-names
                          Use unique names for basic block sections (ELF Only)
  -funique-internal-linkage-names
                          Uniqueify Internal Linkage Symbol Names by appending the MD5 hash of the module path
  -funroll-loops          Turn on loop unroller
  -funsafe-math-optimizations
                          Allow unsafe floating-point math optimizations which may decrease precision
  -fuse-cuid=<value>      Method to generate ID's for compilation units for single source offloading languages CUDA and HIP: 'hash' (ID's generated by hashing file path and command line options) | 'random' (ID's generated as random numbers) | 'none' (disabled). Default is 'hash'. This option will be overridden by option '-cuid=[ID]' if it is specified.
  -fuse-line-directives   Use #line in preprocessed output
  -fvalidate-ast-input-files-content
                          Compute and store the hash of input files used to build an AST. Files with mismatching mtime's are considered valid if both contents is identical
  -fveclib=<value>        Use the given vector functions library
  -fvectorize             Enable the loop vectorization passes
  -fverbose-asm           Generate verbose assembly output
  -fverify-intermediate-code
                          Enable verification of LLVM IR
  -fvirtual-function-elimination
                          Enables dead virtual function elimination optimization. Requires -flto=full
  -fvisibility-dllexport=<value>
                          The visibility for dllexport definitions. If Keep is specified the visibility is not adjusted [-fvisibility-from-dllstorageclass]
  -fvisibility-externs-dllimport=<value>
                          The visibility for dllimport external declarations. If Keep is specified the visibility is not adjusted [-fvisibility-from-dllstorageclass]
  -fvisibility-externs-nodllstorageclass=<value>
                          The visibility for external declarations without an explicit DLL storage class. If Keep is specified the visibility is not adjusted [-fvisibility-from-dllstorageclass]
  -fvisibility-from-dllstorageclass
                          Override the visibility of globals based on their final DLL storage class.
  -fvisibility-global-new-delete-hidden
                          Give global C++ operator new and delete declarations hidden visibility
  -fvisibility-global-new-delete=<value>
                          The visibility for global C++ operator new and delete declarations. If 'source' is specified the visibility is not adjusted
  -fvisibility-inlines-hidden-static-local-var
                          When -fvisibility-inlines-hidden is enabled, static variables in inline C++ member functions will also be given hidden visibility by default
  -fvisibility-inlines-hidden
                          Give inline C++ member functions hidden visibility by default
  -fvisibility-ms-compat  Give global types 'default' visibility and global functions and variables 'hidden' visibility by default
  -fvisibility-nodllstorageclass=<value>
                          The visibility for definitions without an explicit DLL storage class. If Keep is specified the visibility is not adjusted [-fvisibility-from-dllstorageclass]
  -fvisibility=<value>    Set the default symbol visibility for all global definitions
  -fwasm-exceptions       Use WebAssembly style exceptions
  -fwhole-program-vtables Enables whole-program vtable optimization. Requires -flto
  -fwrapv                 Treat signed integer overflow as two's complement
  -fwritable-strings      Store string literals as writable data
  -fxl-pragma-pack        Enable IBM XL #pragma pack handling
  -fxray-always-emit-customevents
                          Always emit __xray_customevent(...) calls even if the containing function is not always instrumented
  -fxray-always-emit-typedevents
                          Always emit __xray_typedevent(...) calls even if the containing function is not always instrumented
  -fxray-always-instrument=<value>
                          DEPRECATED: Filename defining the whitelist for imbuing the 'always instrument' XRay attribute.
  -fxray-attr-list=<value>
                          Filename defining the list of functions/types for imbuing XRay attributes.
  -fxray-function-groups=<value>
                          Only instrument 1 of N groups
  -fxray-ignore-loops     Don't instrument functions with loops unless they also meet the minimum function size
  -fxray-instruction-threshold=<value>
                          Sets the minimum function size to instrument with XRay
  -fxray-instrumentation-bundle=<value>
                          Select which XRay instrumentation points to emit. Options: all, none, function-entry, function-exit, function, custom. Default is 'all'.  'function' includes both 'function-entry' and 'function-exit'.
  -fxray-instrument       Generate XRay instrumentation sleds on function entry and exit
  -fxray-link-deps        Link XRay runtime library when -fxray-instrument is specified (default)
  -fxray-modes=<value>    List of modes to link in by default into XRay instrumented binaries.
  -fxray-never-instrument=<value>
                          DEPRECATED: Filename defining the whitelist for imbuing the 'never instrument' XRay attribute.
  -fxray-selected-function-group=<value>
                          When using -fxray-function-groups, select which group of functions to instrument. Valid range is 0 to fxray-function-groups - 1
  -fzero-call-used-regs=<value>
                          Clear call-used registers upon function return (AArch64/x86 only)
  -fzvector               Enable System z vector language extension
  -F <value>              Add directory to framework include search path
  --gcc-install-dir=<value>
                          Use GCC installation in the specified directory. The directory ends with path components like 'lib{,32,64}/gcc{,-cross}/$triple/$version'. Note: executables (e.g. ld) used by the compiler are not overridden by the selected GCC installation
  --gcc-toolchain=<value> Specify a directory where Clang can find 'include' and 'lib{,32,64}/gcc{,-cross}/$triple/$version'. Clang will use the GCC installation with the largest version
  --gcc-triple=<value>    Search for the GCC installation with the specified triple.
  -gcodeview-command-line Emit compiler path and command line into CodeView debug information
  -gcodeview-ghash        Emit type record hashes in a .debug$H section
  -gcodeview              Generate CodeView debug information
  -gdwarf-2               Generate source-level debug information with dwarf version 2
  -gdwarf-3               Generate source-level debug information with dwarf version 3
  -gdwarf-4               Generate source-level debug information with dwarf version 4
  -gdwarf-5               Generate source-level debug information with dwarf version 5
  -gdwarf32               Enables DWARF32 format for ELF binaries, if debug information emission is enabled.
  -gdwarf64               Enables DWARF64 format for ELF binaries, if debug information emission is enabled.
  -gdwarf                 Generate source-level debug information with the default dwarf version
  -gembed-source          Embed source text in DWARF debug sections
  -gen-reproducer=<value> Emit reproducer on (option: off, crash (default), error, always)
  -gline-directives-only  Emit debug line info directives only
  -gline-tables-only      Emit debug line number tables only
  -gmodules               Generate debug info with external references to clang modules or precompiled headers
  -gno-codeview-command-line
                          Don't emit compiler path and command line into CodeView debug information
  -gno-embed-source       Restore the default behavior of not embedding source text in DWARF debug sections
  -gno-inline-line-tables Don't emit inline line tables.
  --gpu-bundle-output     Bundle output files of HIP device compilation
  --gpu-instrument-lib=<value>
                          Instrument device library for HIP, which is a LLVM bitcode containing __cyg_profile_func_enter and __cyg_profile_func_exit
  --gpu-max-threads-per-block=<value>
                          Default max threads per block for kernel launch bounds for HIP
  --gpu-use-aux-triple-only
                          Prepare '-aux-triple' only without populating '-aux-target-cpu' and '-aux-target-feature'.
  -gpulibc                Link the LLVM C Library for GPUs
  -gsplit-dwarf=<value>   Set DWARF fission mode
  -gstrict-dwarf          Restrict DWARF features to those defined in the specified version, avoiding features from later versions.
  -gz=<value>             DWARF debug sections compression type
  -G <size>               Put objects of at most <size> bytes into small data section (MIPS / Hexagon)
  -g                      Generate source-level debug information
  --help-hidden           Display help for hidden options
  -help                   Display available options
  --hip-device-lib=<value>
                          HIP device library
  --hip-link              Link clang-offload-bundler bundles for HIP
  --hip-path=<value>      HIP runtime installation path, used for finding HIP version and adding HIP include path.
  --hip-version=<value>   HIP version in the format of major.minor.patch
  --hipspv-pass-plugin=<dsopath>
                          path to a pass plugin for HIP to SPIR-V passes.
  --hipstdpar-interpose-alloc
                          Replace all memory allocation / deallocation calls with hipManagedMalloc / hipFree equivalents
  --hipstdpar-path=<value>
                          HIP Standard Parallel Algorithm Acceleration library path, used for finding and implicitly including the library header
  --hipstdpar-prim-path=<value>
                          rocPrim path, required by the HIP Standard Parallel Algorithm Acceleration library, used to implicitly include the rocPrim library
  --hipstdpar-thrust-path=<value>
                          rocThrust path, required by the HIP Standard Parallel Algorithm Acceleration library, used to implicitly include the rocThrust library
  --hipstdpar             Enable HIP acceleration for standard parallel algorithms
  -H                      Show header includes and nesting depth
  -I-                     Restrict all prior -I flags to double-quoted inclusion and remove current directory from include path
  -iapinotes-modules <directory>
                          Add directory to the API notes search path referenced by module name
  -ibuiltininc            Enable builtin #include directories even when -nostdinc is used before or after -ibuiltininc. Using -nobuiltininc after the option disables it
  -idirafter <value>      Add directory to AFTER include search path
  -iframeworkwithsysroot <directory>
                          Add directory to SYSTEM framework search path, absolute paths are relative to -isysroot
  -iframework <value>     Add directory to SYSTEM framework search path
  -imacros <file>         Include macros from file before parsing
  -include-pch <file>     Include precompiled header file
  -include <file>         Include file before parsing
  -index-header-map       Make the next included directory (-I or -F) an indexer header map
  -iprefix <dir>          Set the -iwithprefix/-iwithprefixbefore prefix
  -iquote <directory>     Add directory to QUOTE include search path
  -isysroot <dir>         Set the system root directory (usually /)
  -isystem-after <directory>
                          Add directory to end of the SYSTEM include search path
  -isystem <directory>    Add directory to SYSTEM include search path
  -ivfsoverlay <value>    Overlay the virtual filesystem described by file over the real file system
  -iwithprefixbefore <dir>
                          Set directory to include search path with prefix
  -iwithprefix <dir>      Set directory to SYSTEM include search path with prefix
  -iwithsysroot <directory>
                          Add directory to SYSTEM include search path, absolute paths are relative to -isysroot
  -I <dir>                Add directory to the end of the list of include search paths
  --libomptarget-amdgcn-bc-path=<value>
                          Path to libomptarget-amdgcn bitcode library
  --libomptarget-amdgpu-bc-path=<value>
                          Path to libomptarget-amdgcn bitcode library
  --libomptarget-nvptx-bc-path=<value>
                          Path to libomptarget-nvptx bitcode library
  -L <dir>                Add directory to library search path
  -mabi=quadword-atomics  Enable quadword atomics ABI on AIX (AIX PPC64 only). Uses lqarx/stqcx. instructions.
  -mabicalls              Enable SVR4-style position-independent code (Mips only)
  -maix-small-local-exec-tls
                          Produce a faster access sequence for local-exec TLS variables where the offset from the TLS base is encoded as an immediate operand (AIX 64-bit only). This access sequence is not used for variables larger than 32KB.
  -maix-struct-return     Return all structs in memory (PPC32 only)
  -malign-branch-boundary=<value>
                          Specify the boundary's size to align branches
  -malign-branch=<value>  Specify types of branches to align
  -malign-double          Align doubles to two words in structs (x86 only)
  -maltivec               Enable AltiVec vector initializer syntax
  -mamdgpu-ieee           Sets the IEEE bit in the expected default floating point  mode register. Floating point opcodes that support exception flag gathering quiet and propagate signaling NaN inputs per IEEE 754-2008. This option changes the ABI. (AMDGPU only)
  -mapx-features=<value>  Enable features of APX
  -march=<value>          For a list of available architectures for the target use '-mcpu=help'
  -marm64x<value>         Link as a hybrid ARM64X image
  -mbackchain             Link stack frames through backchain on System Z
  -mbranch-protection=<value>
                          Enforce targets of indirect branches and function returns
  -mbranches-within-32B-boundaries
                          Align selected branches (fused, jcc, jmp) within 32-byte boundary
  -mcabac                 Enable CABAC instructions
  -mcmse                  Allow use of CMSE (Armv8-M Security Extensions)
  -mcode-object-version=<value>
                          Specify code object ABI version. Defaults to 5. (AMDGPU only)
  -mconstructor-aliases   Enable emitting complete constructors and destructors as aliases when possible
  -mcpu=<value>           For a list of available CPUs for the target use '-mcpu=help'
  -mcrbits                Control the CR-bit tracking feature on PowerPC. ``-mcrbits`` (the enablement of CR-bit tracking support) is the default for POWER8 and above, as well as for all other CPUs when optimization is applied (-O2 and above).
  -mcrc                   Allow use of CRC instructions (ARM/Mips only)
  -mcumode                Specify CU wavefront execution mode (AMDGPU only)
  -mdefault-visibility-export-mapping=<value>
                          Mapping between default visibility and export
  -mdouble=<n             Force double to be <n> bits
  -MD                     Write a depfile containing user and system headers
  -meabi <value>          Set EABI type. Default depends on triple)
  -membedded-data         Place constants in the .rodata section instead of the .sdata section even if they meet the -G <size> threshold (MIPS)
  -menable-experimental-extensions
                          Enable use of experimental RISC-V extensions.
  -mexec-model=<value>    Execution model (WebAssembly only)
  -mexecute-only          Disallow generation of data access to code sections (ARM only)
  -mextern-sdata          Assume that externally defined data is in the small data if it meets the -G <size> threshold (MIPS)
  -mfentry                Insert calls to fentry at function entry (x86/SystemZ only)
  -mfix-cmse-cve-2021-35465
                          Work around VLLDM erratum CVE-2021-35465 (ARM only)
  -mfix-cortex-a53-835769 Workaround Cortex-A53 erratum 835769 (AArch64 only)
  -mfix-cortex-a57-aes-1742098
                          Work around Cortex-A57 Erratum 1742098 (ARM only)
  -mfix-cortex-a72-aes-1655431
                          Work around Cortex-A72 Erratum 1655431 (ARM only)
  -mforced-sw-shadow-stack
                          Force using software shadow stack when shadow-stack enabled
  -mfp32                  Use 32-bit floating point registers (MIPS only)
  -mfp64                  Use 64-bit floating point registers (MIPS only)
  -mfpxx                  Avoid FPU mode dependent operations when used with the O32 ABI
  -mframe-chain=<value>   Select the frame chain model used to emit frame records (Arm only).
  -mfunction-return=<value>
                          Replace returns with jumps to ``__x86_return_thunk`` (x86 only, error otherwise)
  -MF <file>              Write depfile output from -MMD, -MD, -MM, or -M to <file>
  -mgeneral-regs-only     Generate code which only uses the general purpose registers (AArch64/x86 only)
  -mglobal-merge          Enable merging of globals
  -mgpopt                 Use GP relative accesses for symbols known to be in a small data section (MIPS)
  -mguard=<value>         Enable or disable Control Flow Guard checks and guard tables emission
  -MG                     Add missing headers to depfile
  -mharden-sls=<value>    Select straight-line speculation hardening scope (ARM/AArch64/X86 only). <arg> must be: all, none, retbr(ARM/AArch64), blr(ARM/AArch64), comdat(ARM/AArch64), nocomdat(ARM/AArch64), return(X86), indirect-jmp(X86)
  -mhvx-ieee-fp           Enable Hexagon HVX IEEE floating-point
  -mhvx-length=<value>    Set Hexagon Vector Length
  -mhvx-qfloat            Enable Hexagon HVX QFloat instructions
  -mhvx=<value>           Enable Hexagon Vector eXtensions
  -mhvx                   Enable Hexagon Vector eXtensions
  -miamcu                 Use Intel MCU ABI
  -mignore-xcoff-visibility
                          Not emit the visibility attribute for asm in AIX OS or give all symbols 'unspecified' visibility in XCOFF object file
  --migrate               Run the migrator
  -mincremental-linker-compatible
                          (integrated-as) Emit an object file which can be used with an incremental linker
  -mindirect-branch-cs-prefix
                          Add cs prefix to call and jmp to indirect thunk
  -mindirect-jump=<value> Change indirect jump instructions to inhibit speculation
  -mios-version-min=<value>
                          Set iOS deployment target
  -mips1                  Equivalent to -march=mips1
  -mips2                  Equivalent to -march=mips2
  -mips32r2               Equivalent to -march=mips32r2
  -mips32r3               Equivalent to -march=mips32r3
  -mips32r5               Equivalent to -march=mips32r5
  -mips32r6               Equivalent to -march=mips32r6
  -mips32                 Equivalent to -march=mips32
  -mips3                  Equivalent to -march=mips3
  -mips4                  Equivalent to -march=mips4
  -mips5                  Equivalent to -march=mips5
  -mips64r2               Equivalent to -march=mips64r2
  -mips64r3               Equivalent to -march=mips64r3
  -mips64r5               Equivalent to -march=mips64r5
  -mips64r6               Equivalent to -march=mips64r6
  -mips64                 Equivalent to -march=mips64
  -MJ <value>             Write a compilation database entry per input
  -mlasx                  Enable Loongson Advanced SIMD Extension (LASX).
  -mllvm=<arg>            Alias for -mllvm
  -mllvm <value>          Additional arguments to forward to LLVM's option processing
  -mlocal-sdata           Extend the -G behaviour to object local data (MIPS)
  -mlong-calls            Generate branches with extended addressability, usually via indirect jumps.
  -mlong-double-128       Force long double to be 128 bits
  -mlong-double-64        Force long double to be 64 bits
  -mlong-double-80        Force long double to be 80 bits, padded to 128 bits for storage
  -mlsx                   Enable Loongson SIMD Extension (LSX).
  -mlvi-cfi               Enable only control-flow mitigations for Load Value Injection (LVI)
  -mlvi-hardening         Enable all mitigations for Load Value Injection (LVI)
  -mmacos-version-min=<value>
                          Set macOS deployment target
  -mmadd4                 Enable the generation of 4-operand madd.s, madd.d and related instructions.
  -mmark-bti-property     Add .note.gnu.property with BTI to assembly files (AArch64 only)
  -MMD                    Write a depfile containing user headers
  -mmemops                Enable generation of memop instructions
  -mmlir <value>          Additional arguments to forward to MLIR's option processing
  -mms-bitfields          Set the default structure layout to be compatible with the Microsoft compiler standard
  -mmsa                   Enable MSA ASE (MIPS only)
  -mmt                    Enable MT ASE (MIPS only)
  -MM                     Like -MMD, but also implies -E and writes to stdout by default
  -mno-abicalls           Disable SVR4-style position-independent code (Mips only)
  -mno-apx-features=<value>
                          Disable features of APX
  -mno-bti-at-return-twice
                          Do not add a BTI instruction after a setjmp or other return-twice construct (Arm/AArch64 only)
  -mno-constructor-aliases
                          Disable emitting complete constructors and destructors as aliases when possible
  -mno-crc                Disallow use of CRC instructions (Mips only)
  -mno-cumode             Specify WGP wavefront execution mode (AMDGPU only)
  -mno-embedded-data      Do not place constants in the .rodata section instead of the .sdata if they meet the -G <size> threshold (MIPS)
  -mno-execute-only       Allow generation of data access to code sections (ARM only)
  -mno-extern-sdata       Do not assume that externally defined data is in the small data if it meets the -G <size> threshold (MIPS)
  -mno-fix-cmse-cve-2021-35465
                          Don't work around VLLDM erratum CVE-2021-35465 (ARM only)
  -mno-fix-cortex-a53-835769
                          Don't workaround Cortex-A53 erratum 835769 (AArch64 only)
  -mno-fix-cortex-a57-aes-1742098
                          Don't work around Cortex-A57 Erratum 1742098 (ARM only)
  -mno-fix-cortex-a72-aes-1655431
                          Don't work around Cortex-A72 Erratum 1655431 (ARM only)
  -mno-fmv                Disable function multiversioning
  -mno-forced-sw-shadow-stack
                          Not force using software shadow stack when shadow-stack enabled
  -mno-gather             Disable generation of gather instructions in auto-vectorization(x86 only)
  -mno-global-merge       Disable merging of globals
  -mno-gpopt              Do not use GP relative accesses for symbols known to be in a small data section (MIPS)
  -mno-hvx-ieee-fp        Disable Hexagon HVX IEEE floating-point
  -mno-hvx-qfloat         Disable Hexagon HVX QFloat instructions
  -mno-hvx                Disable Hexagon Vector eXtensions
  -mno-implicit-float     Don't generate implicit floating point or vector instructions
  -mno-incremental-linker-compatible
                          (integrated-as) Emit an object file which cannot be used with an incremental linker
  -mno-lasx               Disable Loongson Advanced SIMD Extension (LASX).
  -mno-local-sdata        Do not extend the -G behaviour to object local data (MIPS)
  -mno-long-calls         Restore the default behaviour of not generating long calls
  -mno-lsx                Disable Loongson SIMD Extension (LSX).
  -mno-lvi-cfi            Disable control-flow mitigations for Load Value Injection (LVI)
  -mno-lvi-hardening      Disable mitigations for Load Value Injection (LVI)
  -mno-madd4              Disable the generation of 4-operand madd.s, madd.d and related instructions.
  -mno-memops             Disable generation of memop instructions
  -mno-movt               Disallow use of movt/movw pairs (ARM only)
  -mno-ms-bitfields       Do not set the default structure layout to be compatible with the Microsoft compiler standard
  -mno-msa                Disable MSA ASE (MIPS only)
  -mno-mt                 Disable MT ASE (MIPS only)
  -mno-neg-immediates     Disallow converting instructions with negative immediates to their negation or inversion.
  -mno-nvj                Disable generation of new-value jumps
  -mno-nvs                Disable generation of new-value stores
  -mno-odd-spreg          Disable odd single-precision floating point registers
  -mno-outline-atomics    Don't generate local calls to out-of-line atomic operations
  -mno-outline            Disable function outlining (AArch64 only)
  -mno-packets            Disable generation of instruction packets
  -mno-pic-data-is-text-relative
                          Don't assume data segments are relative to text segment
  -mno-regnames           Use only register numbers when writing assembly output
  -mno-relax-pic-calls    Do not produce relaxation hints for linkers to try optimizing PIC call sequences into direct calls (MIPS only)
  -mno-relax              Disable linker relaxation
  -mno-restrict-it        Allow generation of complex IT blocks.
  -mno-save-restore       Disable using library calls for save and restore
  -mno-scatter            Disable generation of scatter instructions in auto-vectorization(x86 only)
  -mno-seses              Disable speculative execution side effect suppression (SESES)
  -mno-stack-arg-probe    Disable stack probes which are enabled by default
  -mno-strict-align       Allow memory accesses to be unaligned (AArch64/LoongArch/RISC-V only)
  -mno-tgsplit            Disable threadgroup split execution mode (AMDGPU only)
  -mno-tls-direct-seg-refs
                          Disable direct TLS access through segment registers
  -mno-tocdata=<value>    Specifies a list of variables to be exempt from the TOC datatransformation.
  -mno-tocdata            This is the default. TOC data transformation is not applied to anyvariables. Only variables specified explicitly in -mtocdata= willhave the TOC data transformation.
  -mno-unaligned-access   Force all memory accesses to be aligned (AArch32/MIPSr6 only)
  -mno-unaligned-symbols  Expect external char-aligned symbols to be without ABI alignment (SystemZ only)
  -mno-wavefrontsize64    Specify wavefront size 32 mode (AMDGPU only)
  -mnocrc                 Disallow use of CRC instructions (ARM only)
  -mnop-mcount            Generate mcount/__fentry__ calls as nops. To activate they need to be patched in.
  -mnvj                   Enable generation of new-value jumps
  -mnvs                   Enable generation of new-value stores
  -modd-spreg             Enable odd single-precision floating point registers
  -module-dependency-dir <value>
                          Directory to dump module dependencies to
  -module-file-info       Provide information about a particular module file
  -momit-leaf-frame-pointer
                          Omit frame pointer setup for leaf functions
  -moutline-atomics       Generate local calls to out-of-line atomic operations
  -moutline               Enable function outlining (AArch64 only)
  -mpacked-stack          Use packed stack layout (SystemZ only).
  -mpackets               Enable generation of instruction packets
  -mpad-max-prefix-size=<value>
                          Specify maximum number of prefixes to use for padding
  -mpic-data-is-text-relative
                          Assume data segments are relative to text segment
  -mprefer-vector-width=<value>
                          Specifies preferred vector width for auto-vectorization. Defaults to 'none' which allows target specific decisions.
  -mprintf-kind=<value>   Specify the printf lowering scheme (AMDGPU only), allowed values are \"hostcall\"(printing happens during kernel execution, this scheme relies on hostcalls which require system to support pcie atomics) and \"buffered\"(printing happens after all kernel threads exit, this uses a printf buffer and does not rely on pcie atomic support)
  -MP                     Create phony target for each dependency (other than main file)
  -mqdsp6-compat          Enable hexagon-qdsp6 backward compatibility
  -MQ <value>             Specify name of main file output to quote in depfile
  -mrecip=<value>         Control use of approximate reciprocal and reciprocal square root instructions followed by <n> iterations of Newton-Raphson refinement. <value> = ( ['!'] ['vec-'] ('rcp'|'sqrt') [('h'|'s'|'d')] [':'<n>] ) | 'all' | 'default' | 'none'
  -mrecip                 Equivalent to '-mrecip=all'
  -mrecord-mcount         Generate a __mcount_loc section entry for each __fentry__ call.
  -mregnames              Use full register names when writing assembly output
  -mrelax-all             (integrated-as) Relax all machine instructions
  -mrelax-pic-calls       Produce relaxation hints for linkers to try optimizing PIC call sequences into direct calls (MIPS only)
  -mrelax                 Enable linker relaxation
  -mrestrict-it           Disallow generation of complex IT blocks. It is off by default.
  -mrtd                   Make StdCall calling convention the default
  -mrvv-vector-bits=<value>
                          Specify the size in bits of an RVV vector register
  -msave-restore          Enable using library calls for save and restore
  -mseses                 Enable speculative execution side effect suppression (SESES). Includes LVI control flow integrity mitigations
  -msign-return-address=<value>
                          Select return address signing scope
  -mskip-rax-setup        Skip setting up RAX register when passing variable arguments (x86 only)
  -msmall-data-limit=<value>
                          Put global and static data smaller than the limit into a special section
  -msoft-float            Use software floating point
  -mstack-alignment=<value>
                          Set the stack alignment
  -mstack-arg-probe       Enable stack probes
  -mstack-probe-size=<value>
                          Set the stack probe size
  -mstack-protector-guard-offset=<value>
                          Use the given offset for addressing the stack-protector guard
  -mstack-protector-guard-reg=<value>
                          Use the given reg for addressing the stack-protector guard
  -mstack-protector-guard-symbol=<value>
                          Use the given symbol for addressing the stack-protector guard
  -mstack-protector-guard=<value>
                          Use the given guard (global, tls) for addressing the stack-protector guard
  -mstackrealign          Force realign the stack at entry to every function
  -mstrict-align          Force all memory accesses to be aligned (AArch64/LoongArch/RISC-V only)
  -msve-vector-bits=<value>
                          Specify the size in bits of an SVE vector register. Defaults to the vector length agnostic value of \"scalable\". (AArch64 only)
  -msvr4-struct-return    Return small structs in registers (PPC32 only)
  -mtargetos=<value>      Set the deployment target to be the specified OS and OS version
  -mtgsplit               Enable threadgroup split execution mode (AMDGPU only)
  -mthread-model <value>  The thread model to use. Defaults to 'posix')
  -mtls-dialect=<value>   Which thread-local storage dialect to use for dynamic accesses of TLS variables
  -mtls-direct-seg-refs   Enable direct TLS access through segment registers (default)
  -mtls-size=<value>      Specify bit size of immediate TLS offsets (AArch64 ELF only): 12 (for 4KB) | 24 (for 16MB, default) | 32 (for 4GB) | 48 (for 256TB, needs -mcmodel=large)
  -mtocdata=<value>       Specifies a list of variables to which the TOC data transformationwill be applied.
  -mtocdata               All suitable variables will have the TOC data transformation applied
  -mtp=<value>            Thread pointer access method. For AArch32: 'soft' uses a function call, or 'tpidrurw', 'tpidruro' or 'tpidrprw' use the three CP15 registers. 'cp15' is an alias for 'tpidruro'. For AArch64: 'tpidr_el0', 'tpidr_el1', 'tpidr_el2', 'tpidr_el3' or 'tpidrro_el0' use the five system registers. 'elN' is an alias for 'tpidr_elN'.
  -mtune=<value>          Only supported on AArch64, PowerPC, RISC-V, SPARC, SystemZ, and X86
  -MT <value>             Specify name of main file output in depfile
  -munaligned-access      Allow memory accesses to be unaligned (AArch32/MIPSr6 only)
  -munaligned-symbols     Expect external char-aligned symbols to be without ABI alignment (SystemZ only)
  -munsafe-fp-atomics     Enable generation of unsafe floating point atomic instructions. May generate more efficient code, but may not respect rounding and denormal modes, and may give incorrect results for certain memory destinations. (AMDGPU only)
  -mvevpu                 Emit VPU instructions for VE
  -MV                     Use NMake/Jom format for the depfile
  -mwavefrontsize64       Specify wavefront size 64 mode (AMDGPU only)
  -mxcoff-build-id=<0xHEXSTRING>
                          On AIX, request creation of a build-id string, \"0xHEXSTRING\", in the string table of the loader section inside the linked binary
  -mxcoff-roptr           Place constant objects with relocatable address values in the RO data section and add -bforceimprw to the linker flags (AIX only)
  -mzos-hlq-clang=<ClangHLQ>
                          High level qualifier for z/OS C++RT side deck datasets
  -mzos-hlq-csslib=<CsslibHLQ>
                          High level qualifier for z/OS CSSLIB dataset
  -mzos-hlq-le=<LeHLQ>    High level qualifier for z/OS Language Environment datasets
  -mzos-sys-include=<SysInclude>
                          Path to system headers on z/OS
  -M                      Like -MD, but also implies -E and writes to stdout by default
  -no-canonical-prefixes  Use relative paths for invoking subcommands
  --no-cuda-include-ptx=<value>
                          Do not include PTX for the following GPU architecture (e.g. sm_35) or 'all'. May be specified more than once.
  --no-cuda-version-check Don't error out if the detected version of the CUDA install is too low for the requested CUDA gpu architecture.
  --no-default-config     Disable loading default configuration files
  --no-gpu-bundle-output  Do not bundle output files of HIP device compilation
  -no-hip-rt              Do not link against HIP runtime libraries
  --no-offload-arch=<value>
                          Remove CUDA/HIP offloading device architecture (e.g. sm_35, gfx906) from the list of devices to compile for. 'all' resets the list to its default value.
  --no-offload-new-driver Don't Use the new driver for offloading compilation.
  --no-system-header-prefix=<prefix>
                          Treat all #include paths starting with <prefix> as not including a system header.
  -nobuiltininc           Disable builtin #include directories
  -nogpuinc               Do not add include paths for CUDA/HIP and do not include the default CUDA/HIP wrapper headers
  -nogpulib               Do not link device library for CUDA/HIP device compilation
  -nohipwrapperinc        Do not include the default HIP wrapper headers and include paths
  -nostdinc++             Disable standard #include directories for the C++ standard library
  --nvptx-arch-tool=<value>
                          Tool used for detecting NVIDIA GPU arch in the system.
  -ObjC++                 Treat source input files as Objective-C++ inputs
  -objcmt-allowlist-dir-path=<value>
                          Only modify files with a filename contained in the provided directory path
  -objcmt-atomic-property Make migration to 'atomic' properties
  -objcmt-migrate-all     Enable migration to modern ObjC
  -objcmt-migrate-annotation
                          Enable migration to property and method annotations
  -objcmt-migrate-designated-init
                          Enable migration to infer NS_DESIGNATED_INITIALIZER for initializer methods
  -objcmt-migrate-instancetype
                          Enable migration to infer instancetype for method result type
  -objcmt-migrate-literals
                          Enable migration to modern ObjC literals
  -objcmt-migrate-ns-macros
                          Enable migration to NS_ENUM/NS_OPTIONS macros
  -objcmt-migrate-property-dot-syntax
                          Enable migration of setter/getter messages to property-dot syntax
  -objcmt-migrate-property
                          Enable migration to modern ObjC property
  -objcmt-migrate-protocol-conformance
                          Enable migration to add protocol conformance on classes
  -objcmt-migrate-readonly-property
                          Enable migration to modern ObjC readonly property
  -objcmt-migrate-readwrite-property
                          Enable migration to modern ObjC readwrite property
  -objcmt-migrate-subscripting
                          Enable migration to modern ObjC subscripting
  -objcmt-ns-nonatomic-iosonly
                          Enable migration to use NS_NONATOMIC_IOSONLY macro for setting property's 'atomic' attribute
  -objcmt-returns-innerpointer-property
                          Enable migration to annotate property with NS_RETURNS_INNER_POINTER
  -objcmt-whitelist-dir-path=<value>
                          Alias for -objcmt-allowlist-dir-path
  -ObjC                   Treat source input files as Objective-C inputs
  -object-file-name=<file>
                          Set the output <file> for debug infos
  --offload-arch=<value>  Specify an offloading device architecture for CUDA, HIP, or OpenMP. (e.g. sm_35). If 'native' is used the compiler will detect locally installed architectures. For HIP offloading, the device architecture can be followed by target ID features delimited by a colon (e.g. gfx908:xnack+:sramecc-). May be specified more than once.
  --offload-compression-level=<value>
                          Compression level for offload device binaries (HIP only)
  --offload-compress      Compress offload device binaries (HIP only)
  --offload-device-only   Only compile for the offloading device.
  --offload-host-device   Compile for both the offloading host and device (default).
  --offload-host-only     Only compile for the offloading host.
  --offload-link          Use the new offloading linker to perform the link job.
  --offload-new-driver    Use the new driver for offloading compilation.
  --offload=<value>       Specify comma-separated list of offloading target triples (CUDA and HIP only)
  -o <file>               Write output to <file>
  -pedantic               Warn on language extensions
  -pg                     Enable mcount instrumentation
  -pipe                   Use pipes between commands, when possible
  --precompile            Only precompile the input
  --pretty-sgf            Emit pretty printed symbol graphs
  -print-diagnostic-options
                          Print all of Clang's warning options
  -print-effective-triple Print the effective target triple
  -print-file-name=<file> Print the full library path of <file>
  -print-ivar-layout      Enable Objective-C Ivar layout bitmap print trace
  -print-libgcc-file-name Print the library path for the currently used compiler runtime library (\"libgcc.a\" or \"libclang_rt.builtins.*.a\")
  -print-library-module-manifest-path
                          Print the path for the C++ Standard library module manifest
  -print-multi-flags-experimental
                          Print the flags used for selecting multilibs (experimental)
  -print-prog-name=<name> Print the full program path of <name>
  -print-resource-dir     Print the resource directory pathname
  -print-rocm-search-dirs Print the paths used for finding ROCm installation
  -print-runtime-dir      Print the directory pathname containing Clang's runtime libraries
  -print-search-dirs      Print the paths used for finding libraries and programs
  -print-supported-cpus   Print supported cpu models for the given target (if target is not specified, it will print the supported cpus for the default target)
  -print-supported-extensions
                          Print supported -march extensions (RISC-V, AArch64 and ARM only)
  -print-target-triple    Print the normalized target triple
  -print-targets          Print the registered targets
  -pthread                Support POSIX threads in generated code
  --ptxas-path=<value>    Path to ptxas (used for compiling CUDA code)
  -P                      Disable linemarker output in -E mode
  -p                      Enable mcount instrumentation with prof
  -Qn                     Do not emit metadata containing compiler name and version
  -Qunused-arguments      Don't emit warning for unused driver arguments
  -Qy                     Emit metadata containing compiler name and version
  -regcall4               Set __regcall4 as a default calling convention to respect __regcall ABI v.4
  -relocatable-pch        Whether to build a relocatable precompiled header
  -resource-dir <value>   The directory which holds the compiler resource files
  -rewrite-legacy-objc    Rewrite Legacy Objective-C source to C++
  -rewrite-objc           Rewrite Objective-C source to C++
  --rocm-device-lib-path=<value>
                          ROCm device library path. Alternative to rocm-path.
  --rocm-path=<value>     ROCm installation path, used for finding and automatically linking required bitcode libraries.
  -Rpass-analysis=<value> Report transformation analysis from optimization passes whose name matches the given POSIX regular expression
  -Rpass-missed=<value>   Report missed transformations by optimization passes whose name matches the given POSIX regular expression
  -Rpass=<value>          Report transformations performed by optimization passes whose name matches the given POSIX regular expression
  -rtlib=<value>          Compiler runtime library to use
  -R<remark>              Enable the specified remark
  -save-stats=<value>     Save llvm statistics.
  -save-stats             Save llvm statistics.
  -save-temps=<value>     Save intermediate compilation results. <arg> can be set to 'cwd' for current working directory, or 'obj' which will save temporary files in the same directory as the final output file
  -save-temps             Alias for --save-temps=cwd
  -serialize-diagnostics <value>
                          Serialize compiler diagnostics to a file
  -shared-libsan          Dynamically link the sanitizer runtime
  --start-no-unused-arguments
                          Don't emit warnings about unused arguments for the following arguments
  -static-libsan          Statically link the sanitizer runtime (Not supported for ASan, TSan or UBSan on darwin)
  -static-openmp          Use the static host OpenMP runtime while linking.
  -std=<value>            Language standard to compile for
  -stdlib++-isystem <directory>
                          Use directory as the C++ standard library include path
  -stdlib=<value>         C++ standard library to use
  -sycl-std=<value>       SYCL language standard to compile for.
  --symbol-graph-dir=<value>
                          Directory in which to emit symbol graphs.
  --system-header-prefix=<prefix>
                          Treat all #include paths starting with <prefix> as including a system header.
  -S                      Only run preprocess and compilation steps
  --target=<value>        Generate code for the given target
  -time                   Time individual commands
  -traditional-cpp        Enable some traditional CPP emulation
  -trigraphs              Process trigraph sequences
  -T <script>             Specify <script> as linker script
  -undef                  undef all system defines
  -unwindlib=<value>      Unwind library to use
  -U <macro>              Undefine macro <macro>
  --verify-debug-info     Verify the binary representation of debug output
  -verify-pch             Load and verify that a pre-compiled header file is not stale
  --version               Print version information
  -vfsoverlay <value>     Overlay the virtual filesystem described by file over the real file system. Additionally, pass this overlay file to the linker if it supports it
  -v                      Show commands to run and use verbose output
  -Wa,<arg>               Pass the comma separated arguments in <arg> to the assembler
  -Wdeprecated            Enable warnings for deprecated constructs and define __DEPRECATED
  -Wl,<arg>               Pass the comma separated arguments in <arg> to the linker
  -Wlarge-by-value-copy   Warn if a function definition returns or accepts an object larger in bytes than a given value
  -working-directory <value>
                          Resolve file paths relative to the specified directory
  -Wp,<arg>               Pass the comma separated arguments in <arg> to the preprocessor
  -Wsystem-headers-in-module=<module>
                          Enable -Wsystem-headers when building <module>
  -Wundef-prefix=<arg>    Enable warnings for undefined macros with a prefix in the comma separated list <arg>
  -W<warning>             Enable the specified warning
  -w                      Suppress all warnings
  -Xanalyzer <arg>        Pass <arg> to the static analyzer
  -Xarch_device <arg>     Pass <arg> to the CUDA/HIP device compilation
  -Xarch_host <arg>       Pass <arg> to the CUDA/HIP host compilation
  -Xassembler <arg>       Pass <arg> to the assembler
  -Xclang=<arg>           Alias for -Xclang
  -Xclang <arg>           Pass <arg> to clang -cc1
  -Xcuda-fatbinary <arg>  Pass <arg> to fatbinary invocation
  -Xcuda-ptxas <arg>      Pass <arg> to the ptxas assembler
  -Xlinker <arg>          Pass <arg> to the linker
  -Xoffload-linker<triple> <arg>
                          Pass <arg> to the offload linkers or the ones idenfied by -<triple>
  -Xopenmp-target=<triple> <arg>
                          Pass <arg> to the target offloading toolchain identified by <triple>.
  -Xopenmp-target <arg>   Pass <arg> to the target offloading toolchain.
  -Xpreprocessor <arg>    Pass <arg> to the preprocessor
  -x <language>           Treat subsequent input files as having type <language>
  -z <arg>                Pass -z <arg> to the linker

dxc compatibility options.:
  --dxv-path=<value>  DXIL validator installation path
  -fspv-target-env=<value>
                      Specify the target environment
  -hlsl-entry <value> Entry point name for hlsl
  /validator-version <value>
                      Override validator version for module. Format: <major.minor>;Default: DXIL.dll version or current internal version

".to_string()
}
