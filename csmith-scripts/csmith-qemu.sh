#! /bin/bash

# Searches for runtime mismatches between the given config and rv64gc

# Invoked using ./csmith-scripts/csmith-qemu.sh <temp folder name> '<gcc-args>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters. Should be ./csmith-scripts/csmith-qemu.sh <temp folder name> '<gcc-args>'"
    exit 1
fi

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

mkdir -p $invocation_location/csmith-discoveries
mkdir -p $invocation_location/csmith-tmp/$1

csmith_tmp=$invocation_location/csmith-tmp/$1

COUNTER=0
while true
do
  # Remove temp files
  rm -f $csmith_tmp/rv64gc-ex.log $csmith_tmp/user-config-ex.log $csmith_tmp/rv64gc-qemu.log $csmith_tmp/user-config-qemu.log

  let COUNTER++

  # Generate a random c program
  $(cat $script_location/csmith.path)/bin/csmith > $csmith_tmp/out.c

  # Compile for the user's config
  if $(cat $script_location/compiler.path) -I$(cat $script_location/csmith.path)/include $2 $csmith_tmp/out.c -o $csmith_tmp/user-config.out 2>&1 | grep "internal compiler error";
  then
    echo "! CONFIG ICE FOUND"
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER-qemu-user-config.c
    continue
  fi

  # Compile for native target
  if gcc -I$(cat $script_location/csmith.path)/include -O1 $csmith_tmp/out.c -o $csmith_tmp/native.out 2>&1 | grep "internal compiler error";
  then
    echo "! NATIVE ICE FOUND"
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER-native-ice.c
    continue
  fi

  # Run each binary with a 1 second timeout
  QEMU_CPU="$($(cat $script_location/scripts.path)/march-to-cpu-opt --get-riscv-tag $csmith_tmp/user-config.out)" timeout -k 0.1 1 $(cat $script_location/qemu.path) $csmith_tmp/user-config.out > $csmith_tmp/user-config-qemu.log
  echo $? > $csmith_tmp/user-config-ex.log
  timeout -k 0.1 1 $csmith_tmp/native.out > $csmith_tmp/native.log
  echo $? > $csmith_tmp/native-ex.log

  # Ensure both finished executing successfully (no timeouts/segfaults/etc)
  if [[ $(cat $csmith_tmp/user-config-ex.log) -eq 0 && $(cat $csmith_tmp/native-ex.log) -eq 0 ]];
  then
    # Check to see if the runtime hash differs
    if [[ $(diff $csmith_tmp/native.log $csmith_tmp/user-config-qemu.log | wc -l) -ne 0 ]];
    then
      echo "! DIFF CONFIRMED. Logged in csmith-discoveries/$1-$COUNTER-qemu.c"
      cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER-qemu.c
      cp $csmith_tmp/user-config-qemu.log $invocation_location/csmith-discoveries/$1-$COUNTER-qemu-diff-gcv.c
      cp $csmith_tmp/native.log $invocation_location/csmith-discoveries/$1-$COUNTER-native-diff-gc.c
    fi
  fi

done
