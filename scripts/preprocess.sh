#! /bin/bash

# Used before creduce to preprocess the file and pull headers into the actual file
# Invoke it from a triage directory (eg. triage-6-120)
# Consumes a raw.c file and emits a red.c file which is ready to be reduced.

# ../scripts/preprocess.sh '<compiler opts>'

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Usage: ../scripts/preprocess.sh '<compiler opts>'"
    exit 1
fi

script_location=$(dirname "$0")

# Relies on compiler.path and csmith.path
if [ ! -f "$(cat $script_location/tools/compiler.path)" ]; then
  echo "compiler path: $(cat $script_location/tools/compiler.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/csmith.path)" ]; then
  echo "csmith path: $(cat $script_location/tools/csmith.path) does not exist."
  exit 1
fi

echo $(cat $script_location/tools/compiler.path) -I$(cat $script_location/tools/csmith.path)/include $1 raw.c -E -o red.c
$(cat $script_location/tools/compiler.path) -I$(cat $script_location/tools/csmith.path)/include $1 raw.c -E -o red.c
echo $1 > compiler-opts.txt
