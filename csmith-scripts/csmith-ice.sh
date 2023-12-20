#! /bin/bash

# Searches for internal compiler errors (ICEs) for the given config

# Invoked using ./csmith-scripts/csmith-ice.sh <temp folder name> '<config>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters"
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

mkdir $invocation_location/csmith-discoveries
mkdir -p $invocation_location/csmith-tmp/$1

csmith_tmp=$invocation_location/csmith-tmp/$1

COUNTER=0
while true
do
  let COUNTER++
  echo $COUNTER-$1
  $(cat $script_location/csmith.path)/bin/csmith > $csmith_tmp/out.c
  if $(cat $script_location/compiler.path) -I$(cat $script_location/csmith.path)/include $2 -S $csmith_tmp/out.c 2>&1 | grep "internal compiler error";
  then
    echo "FAILURE FOUND"
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER.c
  fi
done
