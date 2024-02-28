# docker build . -f arm.Dockerfile
# docker build . -f arm.Dockerfile -q > docker_img.txt
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
RUN git clone https://github.com/patrick-rivos/compiler-fuzz-ci
# Build csmith
WORKDIR /compiler-fuzz-ci
RUN git submodule update --depth 1 --init csmith
RUN apt install -y g++ cmake m4 -y
RUN mkdir csmith-build
WORKDIR /compiler-fuzz-ci/csmith
RUN cmake -DCMAKE_INSTALL_PREFIX=../csmith-build .
RUN nice -n 15 make -j $(nproc) && make install
RUN echo /compiler-fuzz-ci/csmith-build > /compiler-fuzz-ci/scripts/tools/csmith.path
# Build yarpgen
WORKDIR /compiler-fuzz-ci
RUN git submodule update --depth 1 --init yarpgen
RUN mkdir yarpgen-build
WORKDIR /compiler-fuzz-ci/yarpgen
RUN cmake -DCMAKE_INSTALL_PREFIX=../yarpgen-build .
RUN nice -n 15 make -j $(nproc)
# Build QEMU
WORKDIR /compiler-fuzz-ci
RUN git clone --depth 1 --branch v8.2.1 https://gitlab.com/qemu-project/qemu.git
WORKDIR /
RUN mkdir qemu-build
WORKDIR /qemu-build
RUN apt install curl gawk build-essential python3 python3-pip ninja-build meson pkg-config libglib2.0-dev -y
RUN /compiler-fuzz-ci/qemu/configure --target-list=aarch64-linux-user --static
RUN nice -n 15 make -j $(nproc)
RUN nice -n 15 make install
WORKDIR /
RUN mkdir qemu-bin
WORKDIR /qemu-bin
RUN cp /qemu-build/aarch64-linux-user/qemu-aarch64 .
RUN echo /qemu-bin/qemu-aarch64 > /compiler-fuzz-ci/scripts/tools/qemu.path
# Scripts
WORKDIR /compiler-fuzz-ci
RUN echo /compiler-fuzz-ci/basic-scripts > /compiler-fuzz-ci/scripts/tools/scripts.path
# Toolchain
WORKDIR /compiler-fuzz-ci
RUN git clone https://github.com/compiler-explorer/infra.git compiler-explorer-infra
WORKDIR /compiler-fuzz-ci/compiler-explorer-infra
RUN make ce
RUN ./bin/ce_install --enable nightly install compilers/c++/cross/gcc/arm64/nightly trunk
RUN /opt/compiler-explorer/arm64/gcc-trunk/aarch64-unknown-linux-gnu/bin/aarch64-unknown-linux-gnu-gcc -v
# Default to arm gcc trunk
RUN echo /opt/compiler-explorer/arm64/gcc-trunk/aarch64-unknown-linux-gnu/bin/aarch64-unknown-linux-gnu-gcc > /compiler-fuzz-ci/scripts/tools/compiler.path

# Release stage
FROM ubuntu:22.04 as runner
COPY --from=build /opt /opt
COPY --from=build /compiler-fuzz-ci/scripts /compiler-fuzz-ci/scripts
COPY --from=build /compiler-fuzz-ci/basic-scripts /compiler-fuzz-ci/basic-scripts
COPY --from=build /compiler-fuzz-ci/csmith-build /compiler-fuzz-ci/csmith-build
COPY --from=build /compiler-fuzz-ci/yarpgen-build /compiler-fuzz-ci/yarpgen-build
COPY --from=build /qemu-bin /qemu-bin
# Install packages
RUN apt update
RUN apt install software-properties-common -y
RUN add-apt-repository ppa:git-core/ppa -y
RUN apt install python3 python3-pip -y
RUN pip install pyelftools
RUN apt install zip parallel -y
# We're ready to fuzz!
WORKDIR /compiler-fuzz-ci
