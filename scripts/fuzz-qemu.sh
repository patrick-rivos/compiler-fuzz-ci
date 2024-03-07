#! /bin/bash

# Searches for runtime mismatches between the given config and rv64gc

# Invoked using ./scripts/fuzz-qemu.sh <temp folder name> '<compiler-args>'
# Places interesting testcases in the csmith-discoveries folder

if [ "$#" -ne 2 ]; then
    echo "Illegal number of parameters. Should be ./scripts/fuzz-qemu.sh <temp folder name> '<gcc-args>'"
    exit 1
fi

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Env vars
RANDOM_GENERATOR=${RANDOM_GENERATOR:-csmith}
COMPILER=${COMPILER:-gcc}

# Relies on qemu.path scripts.path and csmith.path
if [ ! -f "$(cat $script_location/tools/qemu.path)" ]; then
  echo "qemu path: $(cat $script_location/tools/qemu.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/scripts.path)" ]; then
  echo "scripts path: $(cat $script_location/tools/scripts.path) does not exist."
  exit 1
fi
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
  COMPILER_PATH="$(cat $script_location/tools/gcc.path)"
else
  COMPILER="llvm"
  if [ ! -f "$(cat $script_location/tools/llvm.path)" ]; then
    echo "llvm path: $(cat $script_location/tools/llvm.path) does not exist."
    exit 1
  fi
  COMPILER_PATH="$(cat $script_location/tools/llvm.path)"
fi
# Random generator
if [ "$RANDOM_GENERATOR" == "csmith" ]; then
  RANDOM_GENERATOR="csmith"
  if [ ! -d "$(cat $script_location/tools/csmith.path)" ]; then
    echo "csmith path: $(cat $script_location/tools/csmith.path) does not exist."
    exit 1
  fi
else
  RANDOM_GENERATOR="yarpgen"
  if [ ! -d "$(cat $script_location/tools/yarpgen.path)" ]; then
    echo "yarpgen path: $(cat $script_location/tools/yarpgen.path) does not exist."
    exit 1
  fi
fi

echo "Fuzzing $COMPILER with $RANDOM_GENERATOR"

mkdir -p $invocation_location/csmith-discoveries
mkdir -p $invocation_location/csmith-discoveries/stats
mkdir -p $invocation_location/csmith-tmp/$1

csmith_tmp=$invocation_location/csmith-tmp/$1

COUNTER=0
INVALID_NATIVE_COMPILE_COUNTER=0
INVALID_QEMU_COMPILE_COUNTER=0
INVALID_NATIVE_BINARY_COUNTER=0
INVALID_QEMU_BINARY_COUNTER=0
TIMEOUT_NATIVE_BINARY_COUNTER=0
TIMEOUT_QEMU_BINARY_COUNTER=0
SEGFAULT_NATIVE_BINARY_COUNTER=0
SEGFAULT_QEMU_BINARY_COUNTER=0
INTERESTING_BINARY_COUNTER=0
while true
do
  # Remove temp files
  rm -f $csmith_tmp/rv64gc-ex.log $csmith_tmp/user-config-ex.log $csmith_tmp/rv64gc-qemu.log $csmith_tmp/user-config-qemu.log

  # If more than 10% of testcases are interesting, something is horribly wrong. Exit.
  if [ "$COUNTER" -gt "50" ]; then
    if [ "$COUNTER" -lt "$(( $INTERESTING_BINARY_COUNTER * 100 ))" ]; then
      echo "Abnormally high interesting testcase ratio. Exiting."
      exit 1
    fi
  fi

  let COUNTER++

  # Record stats
  echo "{\"programs_evaluated\":\"$COUNTER\",\"interesting_counter\":\"$INTERESTING_BINARY_COUNTER\",\"invalid_native\":{\"total\":\"$INVALID_NATIVE_BINARY_COUNTER\",\"timeouts\":\"$TIMEOUT_NATIVE_BINARY_COUNTER\",\"segfaults\":\"$SEGFAULT_NATIVE_BINARY_COUNTER\",\"compilations\":\"$INVALID_NATIVE_COMPILE_COUNTER\"},\"invalid_qemu\":{\"total\":\"$INVALID_QEMU_BINARY_COUNTER\",\"timeouts\":\"$TIMEOUT_QEMU_BINARY_COUNTER\",\"segfaults\":\"$SEGFAULT_QEMU_BINARY_COUNTER\",\"compilations\":\"$INVALID_NATIVE_COMPILE_COUNTER\"}}" > csmith-discoveries/stats/$1-stats.json

  # Generate a random c program
  if [ "$RANDOM_GENERATOR" == "csmith" ]; then
    $(cat $script_location/tools/csmith.path)/bin/csmith > $csmith_tmp/out.c
  else
    $(cat $script_location/tools/yarpgen.path)/yarpgen --std=c -o $csmith_tmp > /dev/null
    cat $csmith_tmp/init.h $csmith_tmp/func.c $csmith_tmp/driver.c > $csmith_tmp/out.c
  fi

  # Compile for native target
  timeout 600 gcc -I$(cat $script_location/tools/csmith.path)/include -mcmodel=large -fno-pic -w -fpermissive -O1 -fno-strict-aliasing -fwrapv $csmith_tmp/out.c -o $csmith_tmp/native.out > $csmith_tmp/native-compile-log.txt 2>&1
  echo $? > $csmith_tmp/native-compile-exit-code.txt
  if [[ $(cat $csmith_tmp/native-compile-exit-code.txt) -ne 0 ]];
  then
    echo "! FAILURE TO COMPILE"
    let INVALID_NATIVE_COMPILE_COUNTER++
    let INTERESTING_BINARY_COUNTER++
    mkdir -p $invocation_location/csmith-discoveries/$1-$COUNTER
    cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER/raw.c
    cp $csmith_tmp/native-compile-exit-code.txt $invocation_location/csmith-discoveries/$1-$COUNTER/native-compile-exit-code.txt
    cp $csmith_tmp/native-compile-log.txt $invocation_location/csmith-discoveries/$1-$COUNTER/native-compile-log.txt
    echo "-O1" > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler-opts.txt
    echo "gcc" > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler.txt
    echo "native compiler error" > $invocation_location/csmith-discoveries/$1-$COUNTER/error-type.txt
    continue
  fi

  # Run the binary with a 1 second timeout
  timeout -k 0.1 1 $csmith_tmp/native.out 1 > $csmith_tmp/native.log
  echo $? > $csmith_tmp/native-ex.log

  # Once we've confirmed the native binary executes successfully,
  # compile/run the user's config
  if [[ $(cat $csmith_tmp/native-ex.log) -eq 0 ]];
  then
    # Compile for the user's config (ignore warnings)
    timeout -k 1 600 $COMPILER_PATH -I$(cat $script_location/tools/csmith.path)/include -mcmodel=medany -w -fpermissive -fno-strict-aliasing -fwrapv $2 $csmith_tmp/out.c -o $csmith_tmp/user-config.out -w > $csmith_tmp/user-config-compile-log.txt 2>&1
    echo $? > $csmith_tmp/user-config-compile-exit-code.txt
    if [[ $(cat $csmith_tmp/user-config-compile-exit-code.txt) -ne 0 ]];
    then
      echo "! FAILURE TO COMPILE"
      let INVALID_QEMU_COMPILE_COUNTER++
      let INTERESTING_BINARY_COUNTER++
      mkdir -p $invocation_location/csmith-discoveries/$1-$COUNTER
      cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER/raw.c
      cp $csmith_tmp/user-config-compile-exit-code.txt $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-compile-exit-code.txt
      cp $csmith_tmp/user-config-compile-log.txt $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-compile-log.txt
      echo "$2" > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler-opts.txt
      echo $COMPILER_PATH > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler.txt
      echo "user-config compiler error" > $invocation_location/csmith-discoveries/$1-$COUNTER/error-type.txt
      continue
    fi

    # Run the binary with a 1 second timeout
    QEMU_CPU="$($(cat $script_location/tools/scripts.path)/march-to-cpu-opt --get-riscv-tag $csmith_tmp/user-config.out)" timeout -k 0.1 1 $(cat $script_location/tools/qemu.path) $csmith_tmp/user-config.out 1 > $csmith_tmp/user-config-qemu.log
    echo $? > $csmith_tmp/user-config-ex.log

    # Ensure both finished executing successfully (no timeouts/segfaults/etc)
    if [[ $(cat $csmith_tmp/user-config-ex.log) -eq 0 && $(cat $csmith_tmp/native-ex.log) -eq 0 ]];
    then
      # Check to see if the runtime hash differs
      if [[ $(diff $csmith_tmp/native.log $csmith_tmp/user-config-qemu.log | wc -l) -ne 0 ]];
      then
        echo "! DIFF CONFIRMED. Logged in csmith-discoveries/$1-$COUNTER/raw.c"
	let INTERESTING_BINARY_COUNTER++
        mkdir -p $invocation_location/csmith-discoveries/$1-$COUNTER
        cp $csmith_tmp/out.c $invocation_location/csmith-discoveries/$1-$COUNTER/raw.c
        cp $csmith_tmp/user-config-qemu.log $invocation_location/csmith-discoveries/$1-$COUNTER/qemu-diff-gcv.c
        cp $csmith_tmp/native.log $invocation_location/csmith-discoveries/$1-$COUNTER/native-diff-gc.c
        echo "$2" > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler-opts.txt
        echo $COMPILER_PATH > $invocation_location/csmith-discoveries/$1-$COUNTER/compiler.txt
        echo "user-config runtime diff" > $invocation_location/csmith-discoveries/$1-$COUNTER/error-type.txt
        continue
      fi
    elif [[ $(cat $csmith_tmp/user-config-ex.log) -eq 124 ]];
    then
      let TIMEOUT_QEMU_BINARY_COUNTER++
      let INVALID_QEMU_BINARY_COUNTER++
    elif [[ $(cat $csmith_tmp/user-config-ex.log) -eq 139 ]];
    then
      let SEGFAULT_QEMU_BINARY_COUNTER++
      let INVALID_QEMU_BINARY_COUNTER++
    else
      let INVALID_QEMU_BINARY_COUNTER++
      echo "INVALID QEMU WITH UNKNOWN EXIT CODE: $(cat $csmith_tmp/native-ex.log)"
    fi
  elif [[ $(cat $csmith_tmp/native-ex.log) -eq 124 ]];
  then
    let TIMEOUT_NATIVE_BINARY_COUNTER++
    let INVALID_NATIVE_BINARY_COUNTER++
  elif [[ $(cat $csmith_tmp/native-ex.log) -eq 139 ]];
  then
    let SEGFAULT_NATIVE_BINARY_COUNTER++
    let INVALID_NATIVE_BINARY_COUNTER++
  else
    let INVALID_NATIVE_BINARY_COUNTER++
    echo "INVALID NATIVE WITH UNKNOWN EXIT CODE: $(cat $csmith_tmp/native-ex.log)"
  fi

done
