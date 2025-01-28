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

REDUCED_DIR=${REDUCED_DIR:-"$(pwd)"}

# Make sure compiler-opts.txt is set
if [ ! -f "$invocation_location/compiler-opts.txt" ]; then
  echo "compiler opts file: $invocation_location/compiler-opts.txt does not exist."
  exit 1
fi

# Env vars
COMPILER=${COMPILER:-gcc}

# Compiler
if [ "$COMPILER" == "gcc" ]; then
  COMPILER="gcc"
  if [ ! -f "$(cat $script_location/tools/gcc.path)" ]; then
    echo "gcc path: $(cat $script_location/tools/gcc.path) does not exist."
    exit 1
  fi
  COMPILER_PATH="$(cat $script_location/tools/gcc.path)"
else
  COMPILER="llvm"
  if [ ! -f "$(cat $script_location/tools/llvm.path)" ]; then
    echo "llvm path: $(cat $script_location/tools/llvm.path) does not exist."
    exit 1
  fi
  COMPILER_PATH="$(cat $script_location/tools/llvm.path)"
fi

echo "Reducing using compiler $COMPILER at $COMPILER_PATH"

COMPILER_OPTS="$(cat $invocation_location/compiler-opts.txt) $program -o rv64gcv.out"
# These warnings help prevent creduce from introducing undefined behavior.
# Creduce will gladly read beyond the bounds of an array or lots of other stuff.
# Rejecting programs that fail these warnings keep it in check.
WARNING_OPTS="-Wno-unknown-warning-option -Werror -Wfatal-errors -Wall -Wformat -Wno-parentheses-equality -Wno-constant-conversion -Wno-pointer-compare -Wno-implicit-const-int-float-conversion -Wno-compare-distinct-pointer-types -Wno-constant-logical-operand -Wno-pointer-sign -Wno-self-assign -Wno-bool-operation -Wno-unused-function -Wno-unused-variable -Wno-address -Wno-unused-value -Wno-tautological-compare -Wno-unused-but-set-variable -Wno-pointer-compare"

echo clang $WARNING_OPTS -I$(cat $script_location/tools/csmith.path)/include $program -S
timeout -k 2 2 clang $WARNING_OPTS -I$(cat $script_location/tools/csmith.path)/include $program -S 2>&1 | tee native.log
if [[ $(cat native.log | grep "error" | wc -l) -ne 0 ]];
then
  echo "Error detected (with -Werror -Wfatal-errors)"
  exit 1
fi

echo $COMPILER_PATH -I$(cat $script_location/tools/csmith.path)/include -fsigned-char -fno-strict-aliasing -fwrapv $COMPILER_OPTS -w
timeout -k 2 2 $COMPILER_PATH -I$(cat $script_location/tools/csmith.path)/include -fsigned-char -fno-strict-aliasing -fwrapv $COMPILER_OPTS -w > compile.log 2>&1

cat compile.log

# Check against first line of original compile log if possible
# if [[ -f /scratch/tc-testing/compiler-fuzz-ci/csmith-discoveries/rand_args_10-3704/orig-compile.log ]]; then
#   if [[ $(cat $REDUCED_DIR/orig-compile.log | head -1 | cut -d' '  -f3- | grep -f - compile.log | wc -l) -eq 0 ]]; then
#     echo "Did not match original compiler error"
#     exit 1
#   fi
# fi

if [[ "$(echo $COMPILER_PATH | grep "clang" | wc -l)" -ne 0 ]]; then
  # LLVM
  cat compile.log | grep "PLEASE submit a bug report"
  exit $?
else
  # GCC
  cat compile.log | grep "internal compiler error"
  exit $?
fi
