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
if [ ! -f "$(cat $script_location/tools/gcc.path)" ]; then
  echo "compiler path: $(cat $script_location/tools/gcc.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/csmith.path)" ]; then
  echo "csmith path: $(cat $script_location/tools/csmith.path) does not exist."
  exit 1
fi
if [ ! -f "raw.c" ]; then
  echo "raw.c does not exist."
  exit 1
fi

cat raw.c | sed -E '/#include "init.h"/d' > temp.c && mv temp.c raw-no-includes.c

echo $(cat $script_location/tools/gcc.path) -I$(cat $script_location/tools/csmith.path)/include $1 raw.c -E -o red.c
$(cat $script_location/tools/gcc.path) -I$(cat $script_location/tools/csmith.path)/include $1 raw-no-includes.c -E -o red.c
# Remove __attribute__ ((__malloc__ (* lines since clang doesn't like them https://github.com/llvm/llvm-project/issues/53152
# cat red.c | tac | sed '/__attribute__ ((__malloc__ (/d' | tac > temp.c && mv temp.c red.c
cat red.c | tac | sed '/__attribute__ ((__malloc__ (/,/extern/d' | tac > temp.c && mv temp.c red.c
cat red.c | tac | sed 's/__attribute__ ((__access__ ([^)]*)))//g' | tac > temp.c && mv temp.c red.c

# Remove typedef double _Float64 since GCC doesn't like it
cat red.c | sed -E '/typedef.+_Float/d' > temp.c && mv temp.c red.c
cat red.c | sed -E '/#include "init.h"/d' > temp.c && mv temp.c red.c
cat red.c | sed -E '/# .+/d' > temp.c && mv temp.c red.c

echo $1 > compiler-opts.txt
