# csmith-scripts
A collection of scripts used with csmith to discover and reduce testcases.

Make sure you set *.path files with absolute paths

- compiler.path is the path to your riscv compiler.
- qemu.path is the path to the `qemu-riscv64` binary.
- csmith.path is the path to the csmith build folder.
- scripts.path is the path to the scripts folder in riscv-gnu-toolchain.

## Reduction Tips
creduce and cvise are really good at making testcases that contain UB.

Thankfully cred-qemu.sh has some toggles that reduces the odds of an invalid reduction.

`CLANG_WARNING_CHECK=true creduce ...` This toggle runs the program through clang first to check for warnings. This has helped me with detecting unintialized local variables that gcc ignored.
