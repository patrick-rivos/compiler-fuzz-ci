#! /bin/bash

# Used with creduce to reduce a testcase
# Invoke it from a triage directory (eg. triage-6-120) after running preprocess.sh

# Since AFAIK creduce does not let you pass args in, you need to manually set the COMPILER_OPTS before running

# First run a sanity-check
# ../scripts/cred-ice.sh
# Then reduce
# creduce ../scripts/cred-ice.sh red.c

program=${1:-red.c}

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Relies on compiler.path
if [ ! -f "$(cat $script_location/tools/compiler.path)" ]; then
  echo "compiler path: $(cat $script_location/tools/compiler.path) does not exist."
  exit 1
fi

# Make sure compiler-opts.txt is set
if [ ! -f "$invocation_location/compiler-opts.txt" ]; then
  echo "compiler opts file: $invocation_location/compiler-opts.txt does not exist."
  exit 1
fi

COMPILER=$(cat $script_location/tools/compiler.path)
COMPILER_OPTS="$(cat $invocation_location/compiler-opts.txt)  $program -o rv64gcv.out"
# These warnings help prevent creduce from introducing undefined behavior.
# Creduce will gladly read beyond the bounds of an array or lots of other stuff.
# Rejecting programs that fail these warnings keep it in check.
WARNING_OPTS="-Werror -Wfatal-errors -Wall -Wformat -Wno-constant-conversion -Wno-pointer-compare -Wno-implicit-const-int-float-conversion -Wno-compare-distinct-pointer-types -Wno-constant-logical-operand -Wno-pointer-sign -Wno-self-assign -Wno-bool-operation -Wno-unused-function -Wno-unused-variable -Wno-address -Wno-unused-value -Wno-tautological-compare -Wno-unused-but-set-variable -Wno-pointer-compare"

echo clang $WARNING_OPTS $program
clang $WARNING_OPTS $program 2>&1 | tee native.log
if [[ $(cat native.log | grep "error" | wc -l) -ne 0 ]];
then
  echo "Error detected (with -Werror -Wfatal-errors)"
  exit 1
fi

$COMPILER $COMPILER_OPTS -w 2>&1 | tee compile.log

if [[ "$(echo $COMPILER | grep "clang" | wc -l)" -ne 0 ]]; then
  # LLVM
  cat compile.log | grep "PLEASE submit a bug report"
  exit $?
else
  # GCC
  cat compile.log | grep "internal compiler error"
  exit $?
fi
