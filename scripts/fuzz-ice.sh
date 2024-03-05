#! /bin/bash

# Searches for internal compiler errors (ICEs) for the given config

# Invoked using ./scripts/fuzz-ice.sh <temp folder name> '<compiler-args>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters. Should be ./scripts/fuzz-ice.sh <temp folder name> '<compiler-args>'"
    exit 1
fi

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Relies on compiler.path and csmith.path
if [ ! -f "$(cat $script_location/tools/compiler.path)" ]; then
  echo "compiler path: $(cat $script_location/tools/compiler.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/csmith.path)" ]; then
  echo "csmith path: $(cat $script_location/tools/csmith.path) does not exist."
  exit 1
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

  # Compile to check for ICEs
  $(cat $script_location/tools/compiler.path) -I$(cat $script_location/tools/csmith.path)/include $2 $csmith_tmp/out.c -o $csmith_tmp/user-config.out > $csmith_tmp/user-config-compile-log.txt 2>&1
  echo $? > $csmith_tmp/user-config-compile-exit-code.txt
  if [[ $(cat $csmith_tmp/user-config-compile-exit-code.txt) -ne 0 ]];
  then
    echo "! FAILURE TO COMPILE"
    mkdir -p $invocation_location/csmith-discoveries/$1-$COUNTER
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER/raw.c
    cp $csmith_tmp/user-config-compile-exit-code.txt $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-compile-exit-code.txt
    cp $csmith_tmp/user-config-compile-log.txt $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-compile-log.txt
    echo "$2" > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler-opts.txt
    cat $script_location/tools/compiler.path > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler.txt
    echo "user-config compiler error" > $invocation_location/csmith-discoveries/$1-$COUNTER/error-type.txt
    continue
  fi
done
