#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Usage: $0 $(whoami)"
    exit 1
fi

# 86400 is a 24 hr timeout
timeout 86500 docker run -v <local dir to store finds>:/compiler-fuzz-ci/csmith-discoveries ghcr.io/patrick-rivos/compiler-fuzz-ci:latest sh -c "date > /compiler-fuzz-ci/csmith-discoveries/$1 && nice -n 15 timeout 86400 parallel --link \"RANDOM_GENERATOR=yarpgen ./scripts/fuzz-qemu.sh $1-$(date '+%Y-%m-%d')-{1} {2}\" ::: $(seq 1 $(($(nproc) - 8)) | tr '\n' ' ') ::: '-march=rv64gcv -flto -ftree-vectorize -O3' '-march=rv64gcv_zvl256b -flto -ftree-vectorize -O3' '-march=rv64gcv -flto -O3' '-march=rv64gcv_zvl256b -flto -O3' '-march=rv64gcv -flto -ftree-vectorize -O3 -mtune=generic-ooo' '-march=rv64gcv_zvl256b -flto -ftree-vectorize -O3 -mtune=generic-ooo' '-march=rv64gcv -flto -O3 -mtune=generic-ooo' '-march=rv64gcv_zvl256b -flto -O3 -mtune=generic-ooo' '-march=rv64gcv -flto -ftree-vectorize -O3 -mrvv-vector-bits=zvl' '-march=rv64gcv_zvl256b -flto -ftree-vectorize -O3 -mrvv-vector-bits=zvl' '-march=rv64gcv -flto -O3 -mrvv-vector-bits=zvl' '-march=rv64gcv_zvl256b -flto -O3 -mrvv-vector-bits=zvl' '-march=rv64gcv -flto -ftree-vectorize -O3 -mtune=generic-ooo -mrvv-vector-bits=zvl' '-march=rv64gcv_zvl256b -flto -ftree-vectorize -O3 -mtune=generic-ooo -mrvv-vector-bits=zvl' '-march=rv64gcv -flto -O3 -mtune=generic-ooo -mrvv-vector-bits=zvl' '-march=rv64gcv_zvl256b -flto -O3 -mtune=generic-ooo -mrvv-vector-bits=zvl'"
