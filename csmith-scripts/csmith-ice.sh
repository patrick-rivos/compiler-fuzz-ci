#! /bin/bash

# Searches for internal compiler errors (ICEs) for the given config

# Invoked using ./csmith-scripts/csmith-ice.sh <temp folder name> '<gcc-args>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters. Should be ./csmith-scripts/csmith-ice.sh <temp folder name> '<gcc-args>'"
    exit 1
fi

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Relies on compiler.path and csmith.path
if [ ! -f "$(cat $script_location/compiler.path)" ]; then
  echo "compiler path: $(cat $script_location/compiler.path) does not exist."
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
  echo $COUNTER-$1

  # Generate a random c program
  $(cat $script_location/csmith.path)/bin/csmith > $csmith_tmp/out.c

  # Compile to check for ICEs
  if $(cat $script_location/compiler.path) -I$(cat $script_location/csmith.path)/include $2 -S $csmith_tmp/out.c -o $csmith_tmp/out.s 2>&1 | grep "internal compiler error";
  then
    echo "! FAILURE FOUND"
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER.c
  fi
done
