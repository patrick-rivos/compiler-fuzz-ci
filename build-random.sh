#!/bin/bash

rm -rf build
mkdir build
cd build
RANDOM_CONFIG=$(python ../../random_riscv_config.py)
echo $RANDOM_CONFIG
../configure --prefix=$(pwd) --with-multilib-generator="$RANDOM_CONFIG"
make linux -j$(nproc)
