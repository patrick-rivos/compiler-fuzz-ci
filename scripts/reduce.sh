#! /bin/bash
# Auto reduce testcases

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Should be ./scripts/reduce.sh <discoveries folder name>"
    exit 1
fi

script_location=$(dirname "$0")
invocation_location=$(pwd)

# Assert dependencies are set
if [ ! -f "$(cat $script_location/tools/llvm.path)" ]; then
  echo "llvm path: $(cat $script_location/tools/llvm.path) does not exist."
  exit 1
fi
if [ ! -f "$(cat $script_location/tools/gcc.path)" ]; then
  echo "gcc path: $(cat $script_location/tools/gcc.path) does not exist."
  exit 1
fi
if [ ! -f "$(cat $script_location/tools/qemu.path)" ]; then
  echo "qemu path: $(cat $script_location/tools/qemu.path) does not exist."
  exit 1
fi
if [ ! -d "$(cat $script_location/tools/scripts.path)" ]; then
  echo "scripts path: $(cat $script_location/tools/scripts.path) does not exist."
  exit 1
fi

# Fns
process_dir () {
  script_location=$1
  invocation_location=$2
  local dir=$3
  local dir_name="${dir##*/}"
  echo "Processing: $dir"
  if [ ! -f "$dir/error-type.txt" ]; then
    echo "Error type: $dir/error-type.txt does not exist. Skipping $dir"
    return
  fi

  if [ ! -f "$dir/raw.c" ]; then
    echo "Malformed testcase - missing raw.c. Skipping $dir"
    return
  fi

  if [ ! -f "$dir/compiler-opts.txt" ]; then
    echo "Malformed testcase - missing compiler-opts.txt. Skipping $dir"
    return
  fi

  if [ ! -f "$dir/compiler.txt" ]; then
    echo "Malformed testcase - missing compiler.txt. Skipping $dir"
    return
  fi

  if [ "$(cat $dir/compiler.txt | grep "gcc" | wc -l)" -ne 0 ]; then
    COMPILER="gcc"
  else
    COMPILER="llvm"
  fi

  case "$(cat $dir/error-type.txt)" in
    "user-config runtime diff")
#     echo "Skipping runtime"
    reduce_runtime_diff $script_location $invocation_location $dir $COMPILER
      ;;
    "user-config compiler error")
      if [ ! -f "$dir/qemu-compile-exit-code.txt" ]; then
        echo "Malformed testcase - missing qemu-compile-exit-code.txt for user-config compiler error. Skipping $dir"
        return
      fi

      if [ "$(cat $dir/qemu-compile-log.txt | grep "relocation truncated to fit" | wc -l)" -ne 0 ]; then
        echo "Ignoring relocation failure $dir_name"
	mkdir -p $invocation_location/triage-ice/failed/$dir_name
	echo "Relocation fail" > $invocation_location/triage-ice/failed/$dir_name/fail.txt
	cp $dir/qemu-compile-log.txt $invocation_location/triage-ice/failed/$dir_name/qemu-compile-log.txt
	return
      fi

#       if [ "$(cat $dir/qemu-compile-log.txt | grep "Simple vector VT not representable by simple integer vector VT" | wc -l)" -ne 0 ]; then
        # echo "!! IGNORING COMMON FAIL"
	# mkdir -p $invocation_location/triage-ice/failed/$dir_name
	# echo "Vplan fail" > $invocation_location/triage-ice/failed/$dir_name/fail.txt
# 	return
#       fi

      case "$(cat $dir/qemu-compile-exit-code.txt)" in
        "124") echo "Ignoring timeout"
	echo "Ignoring timeout failure $dir_name"
	mkdir -p $invocation_location/triage-ice/failed/$dir_name
	echo "Timeout fail" > $invocation_location/triage-ice/failed/$dir_name/fail.txt
	cp $dir/qemu-compile-log.txt $invocation_location/triage-ice/failed/$dir_name/qemu-compile-log.txt
	return
	# confirm_timeout_error $script_location $invocation_location $dir $COMPILER
	  ;;
        *)
	reduce_compiler_error $script_location $invocation_location $dir $COMPILER
	# echo "Ignoring ICE"
	  ;;
      esac
      ;;
    "native compiler error") echo "native"
    # reduce_native_compiler_error $script_location $invocation_location $dir
      ;;
    *) echo "Unknown error type: $(cat $dir/error-type.txt)"
      ;;
  esac
}
export -f process_dir

setup_triage_runtime () {
  script_location=$1
  invocation_location=$2
  mkdir -p $invocation_location/triage-runtime
  echo "$invocation_location/triage-runtime/$3"
  mkdir "$invocation_location/triage-runtime/$3"
}
export -f setup_triage_runtime

reduce_runtime_diff () {
  script_location=$1
  invocation_location=$2
  local dir=$3
  local dir_name="${dir##*/}"
  local COMPILER=$4
  echo "Reducing QEMU runtime diff"
  triage_dir=$(setup_triage_runtime $script_location $invocation_location $dir_name)
  if [[ $? -ne 0 ]];
  then
    echo "Failed to create triage folder! $triage_dir"
    return
  fi
  echo "New triage folder: $triage_dir"
  cp $dir/raw.c $triage_dir/raw.c
  cp $dir/compiler-opts.txt $triage_dir/compiler-opts.txt
  cp $dir/compiler.txt $triage_dir/compiler.txt
  cd $triage_dir
  # Preprocess file
  $invocation_location/$script_location/preprocess.sh "$(cat $triage_dir/compiler-opts.txt)"
  # Reduce
  echo "Reducing $COMPILER diff... View progress in $triage_dir/reduce.txt"
  COMPILER=$COMPILER CLANG_WARNING_CHECK=true CLANG_RUN_CHECK=true creduce $invocation_location/$script_location/cred-qemu.sh red.c compiler-opts.txt --timeout 600 > $triage_dir/reduce.txt 2>&1
  # Invoke once after reducing to generate artifacts in dir
  cd $triage_dir
  COMPILER=$COMPILER CLANG_WARNING_CHECK=true CLANG_RUN_CHECK=true timeout -k 10 600 $invocation_location/$script_location/cred-qemu.sh red.c compiler-opts.txt > $triage_dir/verbose-log.txt 2>&1
  if [[ "$(echo $? | grep "0" | wc -l)" -ne 1 ]]; then
    cd $invocation_location
    mkdir -p $triage_dir/../failed
    mv $triage_dir $triage_dir/../failed/$dir_name
    echo "QEMU diff reduction failed: $dir_name Result in triage/failed/$dir_name"
  else
    cd $invocation_location
    echo "Successfully reduced QEMU diff: $dir_name Result in triage/$dir_name"
  fi
#   COMPILER=$COMPILER CLANG_WARNING_CHECK=true CLANG_RUN_CHECK=true timeout -k 10 600 $invocation_location/$script_location/cred-qemu.sh red.c compiler-opts.txt > $triage_dir/verbose-log.txt 2>&1
#   if [[ "$(cat $triage_dir/verbose-log.txt | grep "No diff found" | wc -l)" -ne 0 ]]; then
#     cd $invocation_location
#     mkdir -p $triage_dir/../failed
#     mv $triage_dir $triage_dir/../failed/$dir_name
#     echo "QEMU diff reduction failed: $dir_name Result in triage/failed/$dir_name"
#   else
#     cd $invocation_location
#     echo "Successfully reduced QEMU diff: $dir_name Result in triage/$dir_name"
#   fi
}
export -f reduce_runtime_diff

setup_triage_ice () {
  script_location=$1
  invocation_location=$2
  mkdir -p $invocation_location/triage-ice
  echo "$invocation_location/triage-ice/$3"
  mkdir "$invocation_location/triage-ice/$3"
}
export -f setup_triage_ice

reduce_compiler_error () {
  script_location=$1
  invocation_location=$2
  local dir=$3
  local dir_name="${dir##*/}"
  local COMPILER=$4
  echo "Reducing ICE"
  triage_dir=$(setup_triage_ice $script_location $invocation_location $dir_name)
  if [[ $? -ne 0 ]];
  then
    echo "Failed to create triage folder! $triage_dir"
    return
  fi
  echo "New triage folder: $triage_dir"
  cp $dir/raw.c $triage_dir/raw.c
  cp $dir/compiler-opts.txt $triage_dir/compiler-opts.txt
  cp $dir/qemu-compile-log.txt $triage_dir/orig-compile.log
  cp $dir/compiler.txt $triage_dir/compiler.txt
  cd $triage_dir
  # Preprocess file
  $invocation_location/$script_location/preprocess.sh "$(cat $triage_dir/compiler-opts.txt)"
  # Reduce
  echo "Reducing $COMPILER ice... View progress in $triage_dir/reduce.txt"
  COMPILER=$COMPILER REDUCED_DIR=$triage_dir creduce $invocation_location/$script_location/cred-ice.sh red.c compiler-opts.txt --timeout 600 > reduce.txt 2>&1
  # Invoke once after reducing to generate artifacts in dir
  COMPILER=$COMPILER REDUCED_DIR=$triage_dir timeout -k 10 600 $invocation_location/$script_location/cred-ice.sh red.c compiler-opts.txt > $triage_dir/verbose-log.txt 2>&1
#   if [[ "$(cat $triage_dir/compile.log | wc -l)" -ne 0 ]]; then
#     cd $invocation_location
#     echo "ICE reduction paused: $dir_name Result in triage/$dir_name"
#   else
#     cd $invocation_location
#     mkdir -p $triage_dir/../failed
#     mv $triage_dir $triage_dir/../failed/$dir_name
#     echo "ICE reduction failed: $dir_name Result in triage/failed/$dir_name"
#   fi

  if [[ "$(cat $triage_dir/reduce.txt | grep "C-Reduce cannot run" | wc -l)" -ne 0 ]]; then
    if [[ "$(cat $triage_dir/compile.log | wc -l)" -ne 0 ]]; then
      cd $invocation_location
      mkdir -p $triage_dir/../unrecognized
      mv $triage_dir $triage_dir/../unrecognized/$dir_name
      echo "ICE reduction unrecognized: $dir_name Result in triage/unrecognized/$dir_name"
    else
      cd $invocation_location
      mkdir -p $triage_dir/../failed
      mv $triage_dir $triage_dir/../failed/$dir_name
      echo "ICE reduction failed: $dir_name Result in triage/failed/$dir_name"
    fi
  else
    cd $invocation_location
    echo "Successfully reduced ICE: $dir_name Result in triage/$dir_name"
  fi
}
export -f reduce_compiler_error

confirm_timeout_error () {
  script_location=$1
  invocation_location=$2
  local dir=$3
  local dir_name="${dir##*/}"
  local COMPILER=$4
  echo "Reducing ICE"
  triage_dir=$(setup_triage_ice $script_location $invocation_location $dir_name)
  if [[ $? -ne 0 ]];
  then
    echo "Failed to create triage folder! $triage_dir"
    return
  fi
  echo "New triage folder: $triage_dir"
  cp $dir/raw.c $triage_dir/raw.c
  cp $dir/compiler-opts.txt $triage_dir/compiler-opts.txt
  cp $dir/qemu-compile-exit-code.txt $triage_dir/orig-compile-exit-code.txt
  cp $dir/qemu-compile-log.txt $triage_dir/orig-compile.log
  cp $dir/compiler.txt $triage_dir/compiler.txt
  cd $triage_dir
  # Preprocess file
  $invocation_location/$script_location/preprocess.sh "$(cat $triage_dir/compiler-opts.txt)"
  # Reduce
  echo "Reducing $COMPILER timeout... View progress in $triage_dir/reduce.txt"

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

  timeout -k 1 700 $COMPILER_PATH -mcmodel=medany -w -fpermissive -fno-strict-aliasing -fwrapv -ftime-report red.c $dir/out.c -o $dir/user-config.out -w > $triage_dir/user-config-compile-log.txt 2>&1
  if [[ "$?" -ne 124 ]]; then
    cd $invocation_location
    mkdir -p $triage_dir/../failed
    mv $triage_dir $triage_dir/../failed/$dir_name
    echo "Timeout reduction failed: $dir_name Result in triage/failed/$dir_name"
  else
    cd $invocation_location
    echo "Successfully confirmed timeout error: $dir_name Result in triage/$dir_name"
  fi
}
export -f confirm_timeout_error

echo "Processing tasks in parallel."
find $1 -maxdepth 1 -mindepth 1 -type d | parallel --ungroup --tmpdir /scratch/tmp -j 24 "process_dir $script_location $invocation_location {}"
wait

