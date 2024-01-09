#! /bin/bash

# Used with creduce to reduce a testcase
# Invoke it from a triage directory (eg. triage-6-120) after running preprocess.sh

# First run a sanity-check
# ../csmith-scripts/cred-qemu.sh
# Then reduce
# creduce ../csmith-scripts/cred-qemu.sh red.c

program=${1:-red.c}

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Relies on compiler.path qemu.path scripts.path and csmith.path
if [ ! -f "$(cat $script_location/compiler.path)" ]; then
  echo "compiler path: $(cat $script_location/compiler.path) does not exist."
  exit 1
fi
if [ ! -f "$(cat $script_location/qemu.path)" ]; then
  echo "qemu path: $(cat $script_location/qemu.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/scripts.path)" ]; then
  echo "scripts path: $(cat $script_location/scripts.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/csmith.path)" ]; then
  echo "csmith path: $(cat $script_location/csmith.path) does not exist."
  exit 1
fi

# Make sure compiler-opts.txt is set
if [ ! -f "$invocation_location/compiler-opts.txt" ]; then
  echo "compiler opts file: $invocation_location/compiler-opts.txt does not exist."
  exit 1
fi

CLANG_WARNING_CHECK=${CLANG_WARNING_CHECK:-false}
TIMEOUT_ERROR=${TIMEOUT_ERROR:-false}
SCRIPTS=$(cat $script_location/scripts.path)
COMPILER=$(cat $script_location/compiler.path)
COMPILER_1_OPTS="$(cat $invocation_location/compiler-opts.txt) $program -o user-config.out -fsigned-char"
COMPILER_2_OPTS="-O1 $program -o native.out"
# These warnings help prevent creduce from introducing undefined behavior.
# Creduce will gladly read beyond the bounds of an array or lots of other stuff.
# Rejecting programs that fail these warnings keep it in check.
WARNING_OPTS="-Wformat -Wno-compare-distinct-pointer-types -Wno-overflow -Wuninitialized -Warray-bounds -Wreturn-type"
QEMU=$(cat $script_location/qemu.path)

LOCK_IN_EXIT_CODES=${LOCK_IN_EXIT_CODES:-true}
EXIT_CODE_USER_CONFIG=0
EXIT_CODE_NATIVE=0

if [[ "$CLANG_WARNING_CHECK" = true ]];
then
  echo Checking for warnings with clang.

  CLANG_IGNORE="-Wno-constant-conversion -Wno-unused-value -Wno-tautological-constant-out-of-range-compare -Wno-constant-logical-operand -Wno-tautological-compare -Wno-parentheses-equality -Wno-pointer-sign"

  echo clang $program $WARNING_OPTS $CLANG_IGNORE
  clang $program $WARNING_OPTS $CLANG_IGNORE 2> clang-compile.log
  cat clang-compile.log
  if [[ $(cat clang-compile.log | grep "warning" | wc -l) -ne 0 ]];
  then
    echo "Clang Warning detected"
    exit 1
  fi
fi

echo $COMPILER $COMPILER_1_OPTS $WARNING_OPTS
$COMPILER $COMPILER_1_OPTS $WARNING_OPTS 2> compile-user-opts.log
cat compile-user-opts.log
if [[ $(cat compile-user-opts.log | grep "warning" | wc -l) -ne 0 ]];
then
  echo "Warning detected"
  exit 1
fi

# Ignore warnings from the native compiler
echo gcc $COMPILER_2_OPTS -w
gcc $COMPILER_2_OPTS -w 2> compile-native.log
cat compile-native.log
if [[ $(cat compile-native.log | grep "warning" | wc -l) -ne 0 ]];
then
  echo "Warning detected"
  exit 1
fi

echo "Running QEMU"
echo "QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 1 $QEMU user-config.out"
QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 1 $QEMU user-config.out 2>&1 > user-config-qemu.log
echo $? > user-config-ex.log
echo timeout --verbose -k 0.1 1 ./native.out
timeout --verbose -k 0.1 1 ./native.out 2>&1 > native.log
echo $? > native-ex.log

echo "user-config qemu exit code:"
cat user-config-ex.log
echo "native exit code:"
cat native-ex.log

if [[ "$LOCK_IN_EXIT_CODES" = true ]];
then
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
else
  if [[ $(cat native-ex.log) -eq 124 ]];
  then
    if [[ $(cat user-config-ex.log) -eq 124 ]];
    then
      echo "both killed"
      exit 1
    fi
  fi

  if [[ $(cat native-ex.log) -eq 139 ]];
  then
    echo "native segfaulted"
    exit 1
  fi

  if [[ $(cat user-config-ex.log) -eq 139 ]];
  then
    echo "user_config segfaulted"
    exit 1
  fi
fi



if [[ "$TIMEOUT_ERROR" = true ]];
then
  echo "Checking for qemu timeout"
  if [[ $(diff native.log user-config-qemu.log | wc -l) -ne 0 ]];
  then
   echo "Confirming diff with generous runtime"
   QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 10 $QEMU user-config.out 2>&1 > user-config-qemu.log
   echo $? > user-config-ex.log
   timeout --verbose -k 0.1 10 ./native.out 2>&1 > native.log
   echo $? > native-ex.log
  fi
else
  echo "Rejecting timeouts or other weird exit codes"
  if [[ $(cat native-ex.log) -ne 0 ]];
  then
    echo "Weird exit code for native"
    exit 1
  fi

  if [[ $(cat user-config-ex.log) -ne 0 ]];
  then
    echo "Weird exit code for user-config"
    exit 1
  fi
fi

if [[ $(diff native.log user-config-qemu.log | wc -l) -ne 0 ]];
then
  echo "Diff indicated difference"
  diff native.log user-config-qemu.log
  exit 0
fi

echo "No diff found"
exit 1
