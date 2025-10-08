# check=skip=FromPlatformFlagConstDisallowed

FROM --platform=linux/amd64 debian:bookworm-slim@sha256:7e490910eea2861b9664577a96b54ce68ea3e02ce7f51d89cb0103a6f9c386e0

ENV DEBIAN_FRONTEND="noninteractive"

# Install system CA certificates
RUN apt-get update \
    && apt-get -y install ca-certificates curl gzip \
    && apt-get clean

# Prefer compressed Packages.gz
RUN echo 'Acquire::CompressionTypes::Order:: "gz";' > /etc/apt/apt.conf.d/99compress-gzip

# ---------------------------------------------------------------------------
# N64 Modern SDK
#
# Required for examples 1-2
# ---------------------------------------------------------------------------

# Add the crashoveride95 N64SDK APT repository
RUN echo "deb [trusted=yes] https://crashoveride95.github.io/apt/ ./" | tee /etc/apt/sources.list.d/n64sdk.list

# Update apt and install necessary packages for the N64 SDK
RUN apt-get update \
    && apt-get -y upgrade \
    && apt-get -y install \
        binutils-mips-n64 \
        build-essential \
        gcc-mips-linux-gnu \
        gcc-mips-n64 \
        git \
        libhvq \
        libhvqm \
        libleo \
        libmus \
        libnaudio \
        libnustd \
        libnusys \
        makemask \
        n64sdk \
        newlib-mips-n64 \
    && apt-get clean

# Set environment variables for the SDK
ENV N64_LIBGCCDIR=/opt/crashsdk/lib/gcc/mips64-elf/12.2.0
ENV PATH=/opt/crashsdk/bin:$PATH
ENV ROOT=/etc/n64

# ---------------------------------------------------------------------------
# Rust dependencies
#
# Required for example 2-5
# ---------------------------------------------------------------------------

ENV PATH="/root/.cargo/bin:${PATH}"

# Install rust-up
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable \
    && rustup component add rust-src

# Install N64 build tools
RUN cargo install --locked nust64

WORKDIR /workspace
