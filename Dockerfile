# docker build .
# docker build . -q > docker_img.txt
# docker run

FROM ubuntu:22.04 as run-csmith
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
RUN echo /gcc-fuzz-ci/csmith-build > /gcc-fuzz-ci/csmith-scripts/csmith.path
# Build QEMU
WORKDIR /gcc-fuzz-ci
RUN git submodule update --init riscv-gnu-toolchain
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain
RUN git submodule update --depth 1 --init qemu
RUN mkdir build
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain/build
RUN apt install curl gawk build-essential python3 python3-pip ninja-build meson pkg-config libglib2.0-dev -y
RUN ../configure --prefix=$(pwd)
RUN nice -n 15 make build-qemu -j $(nproc)
RUN echo /gcc-fuzz-ci/riscv-gnu-toolchain/scripts > /gcc-fuzz-ci/csmith-scripts/scripts.path
RUN echo /gcc-fuzz-ci/riscv-gnu-toolchain/build/bin/qemu-riscv64 > /gcc-fuzz-ci/csmith-scripts/qemu.path
# Build compiler
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain
RUN git submodule update --depth 1 --init gcc
RUN git submodule update --depth 1 --init binutils
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain/gcc
RUN git checkout master
WORKDIR /gcc-fuzz-ci/riscv-gnu-toolchain/build
RUN apt install libgmp-dev texinfo bison flex -y
RUN nice -n 15 make linux -j $(nproc)
RUN echo /gcc-fuzz-ci/riscv-gnu-toolchain/build/bin/riscv64-unknown-linux-gnu-gcc > /gcc-fuzz-ci/csmith-scripts/compiler.path
# We're ready to fuzz!
WORKDIR /gcc-fuzz-ci
RUN pip install pyelftools
RUN apt install zip parallel -y
