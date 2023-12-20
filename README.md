# GCC Fuzz
Fuzzing for GCC. Currently 2 main approaches are planned:

# Building random configs
RISC-V has a large state space of extensions.
Using a script similar to the linux kernel's randconfig we should be able to choose a random config and ensure it builds with tip-of-tree.
This is a WIP/still getting set up.

# Fuzzing "stable" configs
Using csmith (a random valid c program generator) we can stress the compiler with random programs.
When we find an interesting case (ICE, execution mismatch) we can use creduce or cvise to reduce the testcase.

I (Patrick) have been doing this with success for a bit now and it has helped find issues with the riscv vector targets (and a generic issue too!)

I recommend focusing on ISA strings with "clean" testsuites (no ICEs or execution fails) since that means every new failure will be novel.

## Getting started
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
```

## Start fuzzing:
Update csmith-scripts compiler.path qemu.path scripts.path with the absolute paths to each of those components.

```
./csmith-scripts/csmith-ice csmith-tmp-1 "-march=rv64gcv -mabi=lp64d -ftree-vectorize -O3"
```

### Fuzz faster (& nicely!):
Running a single script is good, but if you have multiple cores (you probably do!) you can use them all!
```
parallel --lb "nice -n 15 ./csmith-qemu.sh csmith-tmp-{} '-march=rv64gcv -mabi=lp64d -ftree-vectorize -O3'" ::: 1 2 3 4 5 6 7 8
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
  printf("%X\n", a);
}
```

## Reduction steps:

1. Set up csmith-scripts directory

Fill out compiler.path, csmith.path, qemu.path, and scripts.path
[More info](./csmith-scripts/README.md).

2. Create triage directory & copy over the testcase

This will hold the initial testcase (rename it to raw.c) and the reduced testcase (red.c)

3. `cd` into the triage folder
4. Preprocess the initial testcase (raw.c)

`../csmith-scripts/preprocess.sh`

5. Edit `cred-ice.sh` or `cred-qemu.sh` to use the correct compilation options

Ensure the behavior is present by running the script:
`../csmith-scripts/cred-ice.sh` or `../csmith-scripts/cred-ice.sh`

This is a great time to try to reduce the command line args/ISA string. See if removing some extensions still causes the issue to show up.

6. Reduce!

You can use creduce or cvise for this. I prefer creduce so that's what I'll use for the examples, but I use them interchangebly. I think the cli/options are the same for both.

`creduce ../csmith-scripts/cred-ice.sh red.c`

and let it reduce!

Some helpful options:

`creduce ../csmith-scripts/cred-ice.sh red.c --n 12` - Use 12 cores instead of the default 4

`creduce ../csmith-scripts/cred-ice.sh red.c --sllooww` - Try harder to reduce the testcase. Typically takes longer to reduce so I'll reduce it without `--sllooww` and then use `--sllooww` after the initial reduction is done.

# Bug trophy case:
PRs welcome!
## Runtime fails:
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112855
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112801
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112561
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=113087
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112929
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112988
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112932
## ICEs:
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112481
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112535
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112554
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112552
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112733
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112773
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112813
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112852
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112851
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112871
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112854
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112872
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112469
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=112971
https://gcc.gnu.org/bugzilla/show_bug.cgi?id=113001

# Contribute
Have an improvement? PRs are welcome!
