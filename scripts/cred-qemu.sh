#! /bin/bash

# Used with creduce to reduce a testcase
# Invoke it from a triage directory (eg. triage-6-120) after running preprocess.sh

# First run a sanity-check
# ../scripts/cred-qemu.sh
# Then reduce
# creduce ../scripts/cred-qemu.sh red.c

program=${1:-red.c}

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Relies on compiler.path qemu.path scripts.path and csmith.path
if [ ! -f "$(cat $script_location/tools/compiler.path)" ]; then
  echo "compiler path: $(cat $script_location/tools/compiler.path) does not exist."
  exit 1
fi
if [ ! -f "$(cat $script_location/tools/qemu.path)" ]; then
  echo "qemu path: $(cat $script_location/tools/qemu.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/scripts.path)" ]; then
  echo "scripts path: $(cat $script_location/tools/scripts.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/csmith.path)" ]; then
  echo "csmith path: $(cat $script_location/tools/csmith.path) does not exist."
  exit 1
fi

# Make sure compiler-opts.txt is set
if [ ! -f "$invocation_location/compiler-opts.txt" ]; then
  echo "compiler opts file: $invocation_location/compiler-opts.txt does not exist."
  exit 1
fi

EXIT_CODE_USER_CONFIG=0
EXIT_CODE_NATIVE=0
CLANG_WARNING_CHECK=${CLANG_WARNING_CHECK:-false}
SCRIPTS=$(cat $script_location/tools/scripts.path)
COMPILER=$(cat $script_location/tools/compiler.path)
COMPILER_1_OPTS="$(cat $invocation_location/compiler-opts.txt) $program -o user-config.out -fsigned-char -fno-strict-aliasing"
COMPILER_2_OPTS="-O1 $program -o native.out -fno-strict-aliasing"
# These warnings help prevent creduce from introducing undefined behavior.
# Creduce will gladly read beyond the bounds of an array or lots of other stuff.
# Rejecting programs that fail these warnings keep it in check.
WARNING_OPTS="-Wno-unknown-warning-option -Werror -Wfatal-errors -Wall -Wformat -Wno-dangling-pointer -Wno-compare-distinct-pointer-types -Wno-overflow -Wuninitialized -Warray-bounds -Wreturn-type -Wno-unused-function -Wno-unused-variable -Wno-unused-but-set-variable -Wno-unused-value -Wno-address -Wno-bool-compare -Wno-pointer-sign -Wno-bool-operation -Wno-tautological-compare -Wno-self-assign -Wno-implicit-const-int-float-conversion -Wno-constant-conversion -Wno-unused-value -Wno-tautological-constant-out-of-range-compare -Wno-constant-logical-operand -Wno-parentheses-equality -Wno-pointer-sign"
QEMU=$(cat $script_location/tools/qemu.path)

if [[ "$CLANG_WARNING_CHECK" = true ]];
then
  echo Checking for warnings with clang.
  echo clang $program $WARNING_OPTS $CLANG_IGNORE
  clang $program $WARNING_OPTS $CLANG_IGNORE 2> clang-compile.log
  cat clang-compile.log
  if [[ $(cat clang-compile.log | grep "error" | wc -l) -ne 0 ]];
  then
    echo "Clang error detected (with -Werror and -Wfatal-errors)"
    exit 1
  fi
fi

if [[ "$CLANG_RUN_CHECK" = true ]];
then
  echo Checking for warnings with clang.

  echo clang -fsanitize=undefined -fsanitize=memory $program -w -o clang-sanitize.out
  clang -fsanitize=undefined -fsanitize=memory $program -w -o clang-sanitize.out 2> clang-compile.log
  cat clang-compile.log

  echo timeout --verbose -k 0.1 5 ./clang-sanitize.out
  timeout --verbose -k 0.1 5 ./clang-sanitize.out > clang-sanitizer.log 2>&1
  if [[ "$?" -ne 0 || $(cat clang-sanitizer.log | grep "runtime error" | wc -l) -ne 0 ]];
  then
    echo "Runtime error for sanitizer"
    exit 1
  fi
fi

echo $COMPILER $COMPILER_1_OPTS $WARNING_OPTS
$COMPILER $COMPILER_1_OPTS $WARNING_OPTS 2> compile-user-opts.log
cat compile-user-opts.log
if [[ $(cat compile-user-opts.log | grep "error" | wc -l) -ne 0 ]];
then
  echo "Error detected (with -Werror and -Wfatal-errors)"
  exit 1
fi

# Ignore warnings from the native compiler
echo gcc $COMPILER_2_OPTS -w
gcc $COMPILER_2_OPTS -w 2> compile-native.log
cat compile-native.log

echo "Running QEMU"
echo "QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 1 $QEMU user-config.out"
QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 1 $QEMU user-config.out > user-config-qemu.log 2>&1
echo $? > user-config-ex.log
echo timeout --verbose -k 0.1 1 ./native.out
timeout --verbose -k 0.1 1 ./native.out > native.log 2>&1
echo $? > native-ex.log

echo "user-config qemu exit code:"
cat user-config-ex.log
echo "native exit code:"
cat native-ex.log

echo "Exit codes have been locked in, ensuring they match."
if [[ $(cat native-ex.log) -ne $EXIT_CODE_NATIVE ]];
then
echo "Weird exit code for native"
exit 1
fi

if [[ $(cat user-config-ex.log) -ne $EXIT_CODE_USER_CONFIG ]];
then
echo "Weird exit code for user-config"
exit 1
fi

if [[ $(diff native.log user-config-qemu.log | wc -l) -ne 0 ]];
then
  echo "Diff indicated difference"
  diff native.log user-config-qemu.log
  exit 0
fi

echo "No diff found"
exit 1
