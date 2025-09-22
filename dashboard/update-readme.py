readme_text = """# Compiler Fuzz

# Fuzzing "stable" configs
Using csmith (a random valid c program generator) we can stress the compiler with random programs.
When we find an interesting case (ICE, execution mismatch) we can use creduce or cvise to reduce the testcase.

I (Patrick) have been doing this with success for a bit now and it has helped find issues with the riscv vector targets (and a generic issue too!)

I recommend focusing on ISA strings with "clean" testsuites (no ICEs or execution fails) since that means every new failure will be novel.

## Getting started
### Quickstart
There is a docker image if you just want to start fuzzing riscv-gcc.

Example command:
```
export RUNNER_NAME="local"
sudo docker pull ghcr.io/patrick-rivos/compiler-fuzz-ci:latest && sudo docker run -v ~/csmith-discoveries:/compiler-fuzz-ci/csmith-discoveries ghcr.io/patrick-rivos/compiler-fuzz-ci:latest sh -c "date > /compiler-fuzz-ci/csmith-discoveries/$RUNNER_NAME && nice -n 15 parallel --link \\"./scripts/fuzz-qemu.sh $RUNNER_NAME-{1} {2}\\" ::: $(seq 1 $(nproc) | tr '\\n' ' ') ::: '-march=rv64gcv -ftree-vectorize -O3' '-march=rv64gcv_zvl256b -ftree-vectorize -O3' '-march=rv64gcv -O3' '-march=rv64gcv_zvl256b -O3' '-march=rv64gcv -ftree-vectorize -O3 -mtune=generic-ooo' '-march=rv64gcv_zvl256b -ftree-vectorize -O3 -mtune=generic-ooo' '-march=rv64gcv -O3 -mtune=generic-ooo' '-march=rv64gcv_zvl256b -O3 -mtune=generic-ooo'"
```

Command structure:
```
sudo docker pull ghcr.io/patrick-rivos/compiler-fuzz-ci:latest \\   # Clone most recent container
&& sudo docker run \\
-v ~/csmith-discoveries:/compiler-fuzz-ci/csmith-discoveries \\     # Map the container's output directory with the user's desired output. Follows the format -v <SELECTED DIR>:<CONTAINER OUTPUT DIR>
ghcr.io/patrick-rivos/compiler-fuzz-ci:latest \\                    # Run this container
sh -c "date > /compiler-fuzz-ci/csmith-discoveries/$RUNNER_NAME \\  # Record the start time
&& nice -n 15 \\                                                    # Run at a low priority so other tasks preempt the fuzzer
parallel --link \\                                                  # Gnu parallel. Link the args so they get mapped to the core enumeration
\\"./scripts/fuzz-qemu.sh $RUNNER_NAME-{1} {2}\\" \\                  # For each core provide a set of args
::: $(seq 1 $(nproc) | tr '\\n' ' ') \\                              # Enumerate cores
::: '-march=rv64gcv -ftree-vectorize -O3' '-march=rv64gcv_zvl256b -ftree-vectorize -O3' '-march=rv64gcv -O3' '-march=rv64gcv_zvl256b -O3' '-march=rv64gcv -ftree-vectorize -O3 -mtune=generic-ooo' '-march=rv64gcv_zvl256b -ftree-vectorize -O3 -mtune=generic-ooo' '-march=rv64gcv -O3 -mtune=generic-ooo' '-march=rv64gcv_zvl256b -O3 -mtune=generic-ooo'"
# ^ All the compiler flags we're interested in
```

### Build csmith:
```
git submodule update --init csmith
sudo apt install -y g++ cmake m4
mkdir csmith-build
cd csmith
cmake -DCMAKE_INSTALL_PREFIX=../csmith-build .
make && make install
```

### Build riscv-gnu-toolchain:
Bump GCC to use tip-of-tree & build:
```
git submodule update --init riscv-gnu-toolchain
cd riscv-gnu-toolchain
git submodule update --init gcc
cd gcc
git checkout master
cd ..
cd ..
mkdir build-riscv-gnu-toolchain
cd build-riscv-gnu-toolchain
../riscv-gnu-toolchain/configure --prefix=$(pwd) --with-arch=rv64gcv --with-abi=lp64d
make linux -j32
make build-qemu -j32
```

## Start fuzzing:
Update scripts compiler.path qemu.path scripts.path with the absolute paths to each of those components.

```
./scripts/fuzz-ice.sh csmith-tmp-1 "-march=rv64gcv -mabi=lp64d -ftree-vectorize -O3"
```

### Fuzz faster (& nicely!):
Running a single script is good, but if you have multiple cores (you probably do!) you can use them all!
```
parallel --lb "nice -n 15 ./fuzz-qemu.sh csmith-tmp-{} '-march=rv64gcv -mabi=lp64d -ftree-vectorize -O3'" ::: {0..$(nproc)}
```
[gnu parallel](https://www.gnu.org/software/parallel/) makes running multiple copies of a script easy.

`nice -n 15` basically tells linux "this process is low priority".
By setting this, we can leave the fuzzer going in the background and linux will automatically de-prioritize the fuzzer when more important tasks happen (like when building GCC/running a testsuite/terminal sessions/anything)

# Triaging a bug
Once you've found a bug you could submit it directly to bugzilla, but it's pretty big and can probably be reduced in size!

Here's what your bug could look like after reducing it [pr112561](https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112561):
```
int printf(char *, ...);
int a, b, c, e;
short d[7][7] = {};
void main() {
  short f;
  c = 0;
  for (; c <= 6; c++) {
    e |= d[c][c] & 1;
    b &= f & 3;
  }
  printf("%X\\n", a);
}
```

## Reduction steps:

1. Set up scripts directory

Fill out compiler.path, csmith.path, qemu.path, and scripts.path
[More info](./scripts/README.md).

2. Create triage directory & copy over the testcase

This will hold the initial testcase (rename it to raw.c) and the reduced testcase (red.c)

3. `cd` into the triage folder
4. Preprocess the initial testcase (raw.c)

`../scripts/preprocess.sh '<gcc-opts>'`

5. Edit `cred-ice.sh` or `cred-qemu.sh` to use the correct compilation options

Ensure the behavior is present by running the script:
`../scripts/cred-ice.sh` or `../scripts/cred-ice.sh`

This is a great time to try to reduce the command line args/ISA string. Edit compiler-opts.txt and see if removing some extensions still causes the issue to show up.

6. Reduce!

You can use creduce or cvise for this. I prefer creduce so that's what I'll use for the examples, but I use them interchangebly. I think the cli/options are the same for both.

`creduce ../scripts/cred-ice.sh red.c compiler-opts.txt`

and let it reduce!

Some helpful options:

`creduce ../scripts/cred-ice.sh red.c compiler-opts.txt --n 12` - Use 12 cores instead of the default 4

`creduce ../scripts/cred-ice.sh red.c compiler-opts.txt --sllooww` - Try harder to reduce the testcase. Typically takes longer to reduce so I'll reduce it without `--sllooww` and then use `--sllooww` after the initial reduction is done.

cvise can be run with a subset of passes. This is helpful for testcases that tend to reduce to undefined behavior.
More info can be found in [/cvise-passes](/cvise-passes/README)

# Bug trophy case:
"""

if __name__ == "__main__":
    with open("dashboard/miscompiled-bugzilla-reports.md", "r") as f:
        miscompiled_reports = f.read()

    with open("dashboard/ice-bugzilla-reports.md", "r") as f:
        ice_reports = f.read()

    with open("dashboard/other-bugzilla-reports.md", "r") as f:
        other_reports = f.read()

    gcc_text = """## GCC
### Runtime fails:
{}
### ICEs:
{}
### Other:
{}

### Bugs filed over time:
![Bugs filed over time](./dashboard/cumulative_bugzilla_reports.png)
"""

    gcc_text = gcc_text.format(miscompiled_reports, ice_reports, other_reports)
    llvm_text = """## LLVM
### Runtime fails:
1. [RISCV64 miscompile at -O1](https://github.com/llvm/llvm-project/issues/78783)
1. [RISCV64 miscompile at -O2/-O1](https://github.com/llvm/llvm-project/issues/80052)
1. [RISCV64 vector miscompile at -O2](https://github.com/llvm/llvm-project/issues/80910)
1. [RISCV vector zvl256b miscompile at -O2](https://github.com/llvm/llvm-project/issues/82430)
1. [[RISC-V] Miscompile at -O2](https://github.com/llvm/llvm-project/issues/83947)
1. [[RISC-V] Miscompile at -O2](https://github.com/llvm/llvm-project/issues/84350)
1. [[RISC-V] Vector -flto -O2 miscompile](https://github.com/llvm/llvm-project/issues/86620)
1. [[RISC-V][SLP] Sign extension miscompile](https://github.com/llvm/llvm-project/issues/86763)
1. [[SLP] Missing sign extension of demoted type before zero extension](https://github.com/llvm/llvm-project/issues/87011)
1. [[RISC-V][SLPVectorizer] rv64gcv miscompile](https://github.com/llvm/llvm-project/issues/88834)
1. [[RISC-V] Miscompile using rv64gcv](https://github.com/llvm/llvm-project/issues/126974)
1. [[RISC-V] Miscompile on rv64gcv with -O[23]](https://github.com/llvm/llvm-project/issues/132071)
1. [[RISC-V] Miscompile on rv64gcv with -O[23]](https://github.com/llvm/llvm-project/issues/133943)
1. [[RISC-V] Miscompile on rv64gcv with -O3](https://github.com/llvm/llvm-project/issues/134126)
1. [[RISC-V] Miscompile in rv64gcv with -O3 -flto](https://github.com/llvm/llvm-project/issues/134705)
1. [[RISC-V] Miscompile on rv64gcv with -O[23]](https://github.com/llvm/llvm-project/issues/138923)
1. [[RISC-V] Miscompile on -O3 with -flto](https://github.com/llvm/llvm-project/issues/141098)
1. [[RISC-V] Miscompile on -O[1-3]](https://github.com/llvm/llvm-project/issues/142004)

### Internal errors:
1. [RISCV64 backend segfault in RISC-V Merge Base Offset](https://github.com/llvm/llvm-project/issues/78679)
1. [RISCV64 backend "Invalid size request on a scalable vector"](https://github.com/llvm/llvm-project/issues/80744)
1. [[LSR][term-fold] Ensure the simple recurrence is reachable from the current loop](https://github.com/llvm/llvm-project/pull/83085)
1. [[InstCombine] Infinite loop/hang](https://github.com/llvm/llvm-project/issues/83354)
1. [[Pass Manager] Infinite loop of scheduled passes](https://github.com/llvm/llvm-project/issues/83469)
1. [[DAGCombiner][RISC-V] DAGCombiner.cpp:8692: Assertion `Index < ByteWidth && "invalid index requested"' failed.](https://github.com/llvm/llvm-project/issues/83920)
1. [[RISC-V] Segfault during pass 'RISC-V DAG->DAG Pattern Instruction Selection'](https://github.com/llvm/llvm-project/issues/83929)
1. [[InstCombine][RISC-V] UNREACHABLE executed at InstCombineCompares.cpp:2788](https://github.com/llvm/llvm-project/issues/83931)
1. [[LoopVectorize] Assertion `OpType == OperationType::DisjointOp && "recipe cannot have a disjoing flag"' failed.](https://github.com/llvm/llvm-project/issues/87378)
1. [[SLP] Attempted invalid cast from VectorType to FixedVectorType](https://github.com/llvm/llvm-project/issues/87384)
1. [[LoopVectorize][VPlan] Unreachable executed "Unhandled opcode!"](https://github.com/llvm/llvm-project/issues/87394)
1. [[LoopVectorize][VPlan] Assertion `MinBWs.size() == NumProcessedRecipes && "some entries in MinBWs haven't been processed"' failed.](https://github.com/llvm/llvm-project/issues/87407)
1. [[LoopVectorize][VPlan] Assertion "Trying to access a single scalar per part but has multiple scalars per part." failed.](https://github.com/llvm/llvm-project/issues/87410)
1. [[Inline] Assert getOperand() out of range! failed.](https://github.com/llvm/llvm-project/issues/87441)
1. [[RISC-V] Error in backend: Invalid size request on a scalable vector.](https://github.com/llvm/llvm-project/issues/88576)
1. [[VectorCombine] Assertion 'isa<To>(Val) && "cast<Ty>() argument of incompatible type!"' failed.](https://github.com/llvm/llvm-project/issues/88796)
1. [[CodeGen][RISC-V] Assertion `(!MMO->getSize().hasValue() || !getSize().hasValue() || MMO->getSize() == getSize()) && "Size mismatch!"' failed.](https://github.com/llvm/llvm-project/issues/88799)
1. [[LoopVectorize] Assertion 'VecTy.SimpleTy != MVT::INVALID_SIMPLE_VALUE_TYPE && "Simple vector VT not representable by simple integer vector VT!"' failed.](https://github.com/llvm/llvm-project/issues/88802)
1. [[LoopVectorize][VPlan] Found non-header PHI recipe in header - Assertion `verifyVPlanIsValid(*Plan) && "VPlan is invalid"' failed.](https://github.com/llvm/llvm-project/issues/88804)
1. [[RISC-V] Assertion `Idx2 != UINT_MAX && Values.contains(Idx2) && "Expected both indices to be extracted already."' failed](https://github.com/llvm/llvm-project/issues/125269)
1. [[RISC-V] LLVM ERROR: Invalid size request on a scalable vector](https://github.com/llvm/llvm-project/issues/125306)
1. [[SLPVectorizer] Segmentation Fault using opt "-passes=lto<O3>"](https://github.com/llvm/llvm-project/issues/126581)
1. [[RISC-V] RegisterCoalescer: Assertion `A.valno == B.valno && "Cannot overlap different values"' failed.](https://github.com/llvm/llvm-project/issues/134424)
1. [[LoopVectorize] Assertion `isPowerOf2_32(End.getKnownMinValue()) && "Expected End to be a power of 2"' failed.](https://github.com/llvm/llvm-project/issues/134696)
1. [[LoopVectorizer] Assertion `hasKnownScalarFactor(RHS) && "Expected RHS to be a known factor!"' failed.](https://github.com/llvm/llvm-project/issues/137024)
1. [[SLPVectorizer] Instruction does not dominate all uses!](https://github.com/llvm/llvm-project/issues/141265)
1. [[InstCombine] ICmp i1 X, C not simplified as expected. with opt "-passes=lto<O3>"](https://github.com/llvm/llvm-project/issues/142447)

### Compiler Flags Fuzzer
1. [[Clang] Assertion isCurrentFileAST() && "dumping non-AST?" failed. with -module-file-info](https://github.com/llvm/llvm-project/issues/87852)
1. [[Clang][Interp] Assertion 'Offset + sizeof(T) <= Pointee->getDescriptor()->getAllocSize()' failed. with -fexperimental-new-constant-interpreter](https://github.com/llvm/llvm-project/issues/88018)
1. [[Clang] Segfault with -fcoverage-mapping -fcs-profile-generate -fprofile-instr-generate](https://github.com/llvm/llvm-project/issues/88038)
1. [[X86][RISC-V][AARCH64] fatal error: error in backend: Can only embed the module once with -fembed-bitcode -ffat-lto-objects -flto](https://github.com/llvm/llvm-project/issues/88041)
1. [[RISC-V] Unhandled encodeInstruction length! at RISCVMCCodeEmitter.cpp:338 with -fglobal-isel -fstack-protector-all](https://github.com/llvm/llvm-project/issues/88046)
1. [[RISC-V] LLVM ERROR: unable to legalize instruction with -fglobal-isel -finstrument-functions -flto -fuse-ld=lld](https://github.com/llvm/llvm-project/issues/88057)
1. [[X86] LLVM ERROR: cannot select with -fglobal-isel -finstrument-functions -flto](https://github.com/llvm/llvm-project/issues/88058)
1. [[LLD] Unreachable executed with -fsplit-stack](https://github.com/llvm/llvm-project/issues/88061)
1. [[RISC-V] Unresolvable relocation with -fdirect-access-external-data -fstack-protector-all](https://github.com/llvm/llvm-project/issues/88079)
1. [[Clang] Assertion 'Symbol' failed. with -fdebug-macro -gline-directives-only](https://github.com/llvm/llvm-project/issues/88153)
1. [[CodeGen] Assertion 'Offset >= Size' failed. with -mms-bitfields](https://github.com/llvm/llvm-project/issues/88208)
"""
    # Fill in the readme text

    ending_text = """
# Contribute
Have an improvement? PRs are welcome!
"""
    readme_text = readme_text + gcc_text + llvm_text + ending_text

    # Write to README.md
    with open("README.md", "w") as f:
        f.write(readme_text)
