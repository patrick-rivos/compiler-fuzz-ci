# docker build .
# docker build . -q > docker_img.txt
# docker run

FROM ubuntu:22.04 as build
RUN apt update
# Need to update git to use --depth
RUN apt install software-properties-common -y
RUN add-apt-repository ppa:git-core/ppa -y
RUN apt-key adv --recv-keys --keyserver keyserver.ubuntu.com A1715D88E1DF1F24 40976EAF437D05B5 3B4FE6ACC0B21F32 A6616109451BBBF2
RUN apt-get update
RUN apt-get install git -y
# Clone repo
RUN git clone https://github.com/patrick-rivos/gcc-fuzz-ci
# Build csmith
WORKDIR /gcc-fuzz-ci
RUN git submodule update --depth 1 --init csmith
RUN apt install -y g++ cmake m4 -y
RUN mkdir csmith-build
WORKDIR /gcc-fuzz-ci/csmith
RUN cmake -DCMAKE_INSTALL_PREFIX=../csmith-build .
RUN nice -n 15 make -j $(nproc) && make install
RUN echo /gcc-fuzz-ci/csmith-build > /gcc-fuzz-ci/scripts/tools/csmith.path
# Build yarpgen
WORKDIR /gcc-fuzz-ci
RUN git submodule update --depth 1 --init yarpgen
RUN mkdir yarpgen-build
WORKDIR /gcc-fuzz-ci/yarpgen
RUN cmake -DCMAKE_INSTALL_PREFIX=../yarpgen-build .
RUN nice -n 15 make -j $(nproc)
# Build QEMU
WORKDIR /gcc-fuzz-ci
RUN git submodule update --init riscv-gnu-toolchain
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain
RUN git submodule update --depth 1 --init qemu
WORKDIR /
RUN mkdir riscv-gnu-toolchain-build
WORKDIR /riscv-gnu-toolchain-build
RUN apt install curl gawk build-essential python3 python3-pip ninja-build meson pkg-config libglib2.0-dev -y
RUN /gcc-fuzz-ci/riscv-gnu-toolchain/configure --prefix=$(pwd)
RUN nice -n 15 make build-qemu -j $(nproc)
RUN echo /gcc-fuzz-ci/riscv-gnu-toolchain/scripts > /gcc-fuzz-ci/scripts/tools/scripts.path
RUN echo /riscv-gnu-toolchain-build/bin/qemu-riscv64 > /gcc-fuzz-ci/scripts/tools/qemu.path
# Build gcc
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain
RUN git submodule update --depth 1 --init gcc
RUN git submodule update --depth 1 --init binutils
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain/gcc
RUN git checkout master
RUN curl https://patchwork.sourceware.org/project/gcc/patch/20240116221914.267015-1-gkm@rivosinc.com/mbox/ > comma_op_fix.patch
RUN git config --global user.email "patrick@rivosinc.com"
RUN git config --global user.name "Patrick O'Neill"
RUN git am comma_op_fix.patch
WORKDIR /riscv-gnu-toolchain-build
RUN apt install libgmp-dev texinfo bison flex -y
RUN nice -n 15 make linux -j $(nproc)
RUN echo /riscv-gnu-toolchain-build/bin/riscv64-unknown-linux-gnu-gcc > /gcc-fuzz-ci/scripts/tools/gcc.path
# Build llvm
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain
RUN git submodule update --depth 1 --init llvm
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain/llvm
RUN git checkout main
WORKDIR /riscv-gnu-toolchain-build
RUN nice -n 15 make stamps/build-llvm-linux -j $(nproc)
RUN echo /riscv-gnu-toolchain-build/bin/clang > /gcc-fuzz-ci/scripts/tools/llvm.path
# Default to gcc
RUN cat /gcc-fuzz-ci/scripts/tools/gcc.path > /gcc-fuzz-ci/scripts/tools/compiler.path

# Release stage
FROM ubuntu:22.04 as runner
COPY --from=build /riscv-gnu-toolchain-build/bin /riscv-gnu-toolchain-build/bin
COPY --from=build /riscv-gnu-toolchain-build/build-glibc-linux-headers /riscv-gnu-toolchain-build/build-glibc-linux-headers
COPY --from=build /riscv-gnu-toolchain-build/build-gdb-linux /riscv-gnu-toolchain-build/build-gdb-linux
COPY --from=build /riscv-gnu-toolchain-build/include /riscv-gnu-toolchain-build/include
COPY --from=build /riscv-gnu-toolchain-build/lib /riscv-gnu-toolchain-build/lib
COPY --from=build /riscv-gnu-toolchain-build/libexec /riscv-gnu-toolchain-build/libexec
COPY --from=build /riscv-gnu-toolchain-build/riscv64-unknown-linux-gnu /riscv-gnu-toolchain-build/riscv64-unknown-linux-gnu
COPY --from=build /riscv-gnu-toolchain-build/scripts /riscv-gnu-toolchain-build/scripts
COPY --from=build /riscv-gnu-toolchain-build/share /riscv-gnu-toolchain-build/share
COPY --from=build /riscv-gnu-toolchain-build/sysroot /riscv-gnu-toolchain-build/sysroot
COPY --from=build /gcc-fuzz-ci/scripts /gcc-fuzz-ci/scripts
COPY --from=build /gcc-fuzz-ci/riscv-gnu-toolchain/scripts /gcc-fuzz-ci/riscv-gnu-toolchain/scripts
COPY --from=build /gcc-fuzz-ci/csmith-build /gcc-fuzz-ci/csmith-build
COPY --from=build /gcc-fuzz-ci/yarpgen-build /gcc-fuzz-ci/yarpgen-build
# Install packages
RUN apt update
RUN apt install software-properties-common -y
RUN add-apt-repository ppa:git-core/ppa -y
RUN apt install python3 python3-pip -y
RUN pip install pyelftools
RUN apt install zip parallel -y
# We're ready to fuzz!
WORKDIR /gcc-fuzz-ci
