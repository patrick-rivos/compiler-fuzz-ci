#! /bin/bash

# Searches for internal compiler errors (ICEs) for the given config

# Invoked using ./scripts/fuzz-ice.sh <temp folder name> '<compiler-args>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Should be ./scripts/fuzz-ice.sh <temp folder name>"
    exit 1
fi

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Relies on csmith.path
if [ ! -d "$(cat $script_location/tools/csmith.path)" ]; then
  echo "csmith path: $(cat $script_location/tools/csmith.path) does not exist."
  exit 1
fi

# Compiler
if [ "$COMPILER" == "gcc" ]; then
  COMPILER="gcc"
  if [ ! -f "$(cat $script_location/tools/gcc.path)" ]; then
    echo "gcc path: $(cat $script_location/tools/gcc.path) does not exist."
    exit 1
  fi
  OPTION_GENERATOR_OPTS="-g -m"
  COMPILER_PATH="$(cat $script_location/tools/gcc.path)"
else
  COMPILER="llvm"
  if [ ! -f "$(cat $script_location/tools/llvm.path)" ]; then
    echo "llvm path: $(cat $script_location/tools/llvm.path) does not exist."
    exit 1
  fi
  OPTION_GENERATOR_OPTS="-l -m"
  COMPILER_PATH="$(cat $script_location/tools/llvm.path)"
fi

mkdir -p $invocation_location/csmith-discoveries
mkdir -p $invocation_location/csmith-tmp/$1

csmith_tmp=$invocation_location/csmith-tmp/$1

COUNTER=0
while true
do
  # Remove temp files
  rm -f $csmith_tmp/rv64gc-ex.log $csmith_tmp/user-config-ex.log $csmith_tmp/rv64gc-qemu.log $csmith_tmp/user-config-qemu.log

  let COUNTER++
  echo $COUNTER-$1

  # Generate a random c program
  $(cat $script_location/tools/csmith.path)/bin/csmith > $csmith_tmp/out.c

  RUST_LOG=off /scratch/tc-testing/compiler-fuzz-ci/compiler_flags_gen/target/release/compiler_flags_gen $OPTION_GENERATOR_OPTS > $csmith_tmp/compiler-opts.txt

  # Compile to check for ICEs
  $COMPILER_PATH -I$(cat $script_location/tools/csmith.path)/include $(cat $csmith_tmp/compiler-opts.txt) $csmith_tmp/out.c -o $csmith_tmp/user-config.out > $csmith_tmp/user-config-compile-log.txt 2>&1
  echo $? > $csmith_tmp/user-config-compile-exit-code.txt
  if [[ $(cat $csmith_tmp/user-config-compile-exit-code.txt) -ne 0 ]];
  then
    echo "! FAILURE TO COMPILE"
    mkdir -p $invocation_location/csmith-discoveries/$1-$COUNTER
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER/raw.c
    cp $csmith_tmp/user-config-compile-exit-code.txt $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-compile-exit-code.txt
    cp $csmith_tmp/user-config-compile-log.txt $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-compile-log.txt
    cp $csmith_tmp/compiler-opts.txt $invocation_location/csmith-discoveries/$1-$COUNTER/compiler-opts.txt
    echo $COMPILER_PATH > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler.txt
    echo "user-config compiler error" > $invocation_location/csmith-discoveries/$1-$COUNTER/error-type.txt
    continue
  fi
done
