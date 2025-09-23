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
        gcc_miscompiles = f.read()

    with open("dashboard/ice-bugzilla-reports.md", "r") as f:
        gcc_ices = f.read()

    with open("dashboard/other-bugzilla-reports.md", "r") as f:
        gcc_other = f.read()

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

    gcc_text = gcc_text.format(gcc_miscompiles, gcc_ices, gcc_other)

    with open("dashboard/llvm-miscompile-issues.md", "r") as f:
        llvm_miscompiles = f.read()

    with open("dashboard/llvm-ice-issues.md", "r") as f:
        llvm_ices = f.read()

    llvm_text = """## LLVM
### Runtime fails:
{}
### ICEs:
{}

### Bugs filed over time:
![Bugs filed over time](./dashboard/cumulative_llvm_github_issues.png)
"""
    llvm_text = llvm_text.format(llvm_miscompiles, llvm_ices)

    ending_text = """
# Contribute
Have an improvement? PRs are welcome!
"""
    readme_text = readme_text + gcc_text + llvm_text + ending_text

    # Write to README.md
    with open("README.md", "w") as f:
        f.write(readme_text)
