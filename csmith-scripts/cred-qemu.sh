#! /bin/bash

# Used with creduce to reduce a testcase
# Invoke it from a triage directory (eg. triage-6-120) after running preprocess.sh

# Since AFAIK creduce does not let you pass args in, you need to manually set the COMPILER_1_OPTS before running

# First run a sanity-check
# ../csmith-scripts/cred-qemu.sh
# Then reduce
# creduce ../csmith-scripts/cred-qemu.sh red.c

program=${1:-red.c}

script_location=$(dirname "$0")

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

TIMEOUT_ERROR=${TIMEOUT_ERROR:-false}
SCRIPTS=$(cat $script_location/scripts.path)
COMPILER=$(cat $script_location/compiler.path)
COMPILER_1_OPTS="-march=rv64gcv -mabi=lp64d -O3 $program -o user-config.out"
COMPILER_2_OPTS="-march=rv64gc -mabi=lp64d -O3 $program -o rv64gc.out"
# These warnings help prevent creduce from introducing undefined behavior.
# Creduce will gladly read beyond the bounds of an array or lots of other stuff.
# Rejecting programs that fail these warnings keep it in check.
WARNING_OPTS="-Wformat -Wno-compare-distinct-pointer-types -Wno-overflow -Wuninitialized -Warray-bounds -Wreturn-type -Wno-incompatible-pointer-types"
QEMU=$(cat $script_location/qemu.path)

LOCK_IN_EXIT_CODES=${LOCK_IN_EXIT_CODES:-true}
EXIT_CODE_user-config=0
EXIT_CODE_RV64GC=0

echo $COMPILER $COMPILER_1_OPTS $WARNING_OPTS
$COMPILER $COMPILER_1_OPTS $WARNING_OPTS 2> comp_output.log
cat comp_output.log
if [[ $(cat comp_output.log | grep "warning" | wc -l) -ne 0 ]];
then
  echo "Warning detected"
  exit 1
fi

echo $COMPILER $COMPILER_2_OPTS $WARNING_OPTS
$COMPILER $COMPILER_2_OPTS $WARNING_OPTS 2> comp_output.log
cat comp_output.log
if [[ $(cat comp_output.log | grep "warning" | wc -l) -ne 0 ]];
then
  echo "Warning detected"
  exit 1
fi

echo "Running QEMU"
echo "QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 1 $QEMU user-config.out"
QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 1 $QEMU user-config.out 2>&1 > user-config-qemu.log
echo $? > user-config-ex.log
echo QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag rv64gc.out)" timeout --verbose -k 0.1 1 $QEMU rv64gc.out
QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag rv64gc.out)" timeout --verbose -k 0.1 1 $QEMU rv64gc.out 2>&1 > rv64gc-qemu.log
echo $? > rv64gc-ex.log

echo "user-config qemu exit code:"
cat user-config-ex.log
echo "rv64gc qemu exit code:"
cat rv64gc-ex.log

if [[ "$LOCK_IN_EXIT_CODES" = true ]];
then
  echo "Exit codes have been locked in, ensuring they match."
  if [[ $(cat rv64gc-ex.log) -ne $EXIT_CODE_RV64GC ]];
  then
    echo "Weird exit code for rv64gc"
    exit 1
  fi

  if [[ $(cat user-config-ex.log) -ne $EXIT_CODE_user-config ]];
  then
    echo "Weird exit code for user-config"
    exit 1
  fi
else
  if [[ $(cat rv64gc-ex.log) -eq 124 ]];
  then
    if [[ $(cat user-config-ex.log) -eq 124 ]];
    then
      echo "both killed"
      exit 1
    fi
  fi

  if [[ $(cat rv64gc-ex.log) -eq 139 ]];
  then
    echo "rv64gc segfaulted"
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
  if [[ $(diff rv64gc-qemu.log user-config-qemu.log | wc -l) -ne 0 ]];
  then
   echo "Confirming diff with generous runtime"
   QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout --verbose -k 0.1 10 $QEMU user-config.out 2>&1 > user-config-qemu.log
   echo $? > user-config-ex.log
   QEMU_CPU="$($SCRIPTS/march-to-cpu-opt --get-riscv-tag rv64gc.out)" timeout --verbose -k 0.1 10 $QEMU rv64gc.out 2>&1 > rv64gc-qemu.log
   echo $? > rv64gc-ex.log
  fi
else
  echo "Rejecting timeouts or other weird exit codes"
  if [[ $(cat rv64gc-ex.log) -ne 0 ]];
  then
    echo "Weird exit code for rv64gc"
    exit 1
  fi

  if [[ $(cat user-config-ex.log) -ne 0 ]];
  then
    echo "Weird exit code for user-config"
    exit 1
  fi
fi

if [[ $(diff rv64gc-qemu.log user-config-qemu.log | wc -l) -ne 0 ]];
then
  echo "Diff indicated difference"
  diff rv64gc-qemu.log user-config-qemu.log
  exit 0
fi

echo "No diff found"
exit 1
