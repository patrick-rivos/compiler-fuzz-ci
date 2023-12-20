#! /bin/bash

# Searches for runtime mismatches between the given config and rv64gc

# Invoked using ./csmith-scripts/csmith-qemu.sh <temp folder name> '<config>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
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

mkdir $invocation_location/csmith-discoveries
mkdir -p $invocation_location/csmith-tmp/$1

csmith_tmp=$invocation_location/csmith-tmp/$1

COUNTER=0
while true
do
  # Remove temp files
  rm $csmith_tmp/rv64gc-ex.log $csmith_tmp/user-config-ex.log $csmith_tmp/rv64gc-qemu.log $csmith_tmp/user-config-qemu.log

  let COUNTER++

  # Generate a random c program
  /scratch/tc-testing/csmith/build/bin/csmith > $csmith_tmp/out.c

  # Compile for the user's config
  if $(cat /scratch/tc-testing/csmith/compiler.path) -I/scratch/tc-testing/csmith/build/include $2 $csmith_tmp/out.c -o $csmith_tmp/user-config.out 2>&1 | grep "internal compiler error";
  then
    echo "! CONFIG ICE FOUND"
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER-qemu-user-config.c
    continue
  fi

  # Compile for rv64gc
  if $(cat /scratch/tc-testing/csmith/compiler.path) -I/scratch/tc-testing/csmith/build/include -march=rv64gc -mabi=lp64d -O3 $csmith_tmp/out.c -o $csmith_tmp/rv64gc.out 2>&1 | grep "internal compiler error";
  then
    echo "! RV64GC ICE FOUND"
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER-qemu-rv64gc.c
    continue
  fi

  # Run each binary with a 1 second timeout
  QEMU_CPU="$($(cat /scratch/tc-testing/csmith/scripts.path)/march-to-cpu-opt --get-riscv-tag user-config.out)" timeout -k 0.1 1 $(cat /scratch/tc-testing/csmith/qemu.path) $csmith_tmp/user-config.out > $csmith_tmp/user-config-qemu.log
  echo $? > $csmith_tmp/user-config-ex.log
  QEMU_CPU="$($(cat /scratch/tc-testing/csmith/scripts.path)/march-to-cpu-opt --get-riscv-tag rv64gc.out)" timeout -k 0.1 1 $(cat /scratch/tc-testing/csmith/qemu.path) $csmith_tmp/rv64gc.out > $csmith_tmp/rv64gc-qemu.log
  echo $? > $csmith_tmp/rv64gc-ex.log

  # Ensure both finished executing successfully (no timeouts/segfaults/etc)
  if [[ $(cat user-config-ex.log) -eq 0 && $(cat rv64gc-ex.log) -eq 0 ]];
  then
    # Check to see if the runtime hash differs
    if [[ $(diff rv64gc-qemu.log user-config-qemu.log | wc -l) -ne 0 ]];
    then
      echo "! DIFF CONFIRMED. Logged in csmith-discoveries/$1-$COUNTER-qemu.c"
      cp out.c $invocation_location/csmith-discoveries/$1-$COUNTER-qemu.c
      cp user-config-qemu.log $invocation_location/csmith-discoveries/$1-$COUNTER-qemu-diff-gcv.c
      cp rv64gc-qemu.log $invocation_location/csmith-discoveries/$1-$COUNTER-qemu-diff-gc.c
    fi
  fi

done
