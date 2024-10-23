# Example 5 - Slide Tool

This example builds upon [04-peripheral-access](../examples/04-peripheral-access) by introducing some basic graphics and interactivity. We'll see that it's not too difficult to get the Nintendo 64 to display images and text using Rust.

## Acknowledgements

This example is heavily inspired by work done by [Tristan Miller](https://github.com/monocasa):

* [n64-slide-apr](https://github.com/monocasa/n64-slides-apr) - Rust Meetup N64 slides from Apr 2019
* [rs64-periph](https://github.com/monocasa/rs64-periph) - Memory-mapped I/O (MMIO) definitions
* [rs64-rt](https://github.com/monocasa/rs64-rt) - Minimal startup / runtime

This has been to be fully self-contained, and built using [nust64](https://github.com/rust-n64/nust64).

## Prerequisites

Install `nust64` and `rs64-romtool`:

```
cargo install nust64
cargo install rs64-romtool
```

These are used to convert ELF to an N64 binary, and to calculate a new checksum for the ROM once we've appended our data blob, respectively.

### IPL3

As discussed in previous examples, Nintendo 64 ROMS require a binary blob of bootcode, known as the IPL3.

Depending on your choice of IPL3 binary, you may need to be update [.cargo/config.toml](.cargo/config.toml) with the necessary path to your IPL3 binary.

## Build

A simple Makefile has been included in this project, meaning the easiest way to build the ROM is to run `make`.

This will ensure that the contents of [fs](./fs) are correctly appended to the ROM, and the checksum is recalculated using `rs64-romtool`.

The final ROM will be located at `target/mips-nintendo64-none/release/slide-tool-final.z64`

### Manual Build

To build manually using `cargo` and `rs64romtool`:

```
cargo run --release
```

This will produce the N64 base ROM, located at `target/mips-nintendo64-none/release/slide-tool.z64`. This contains all of the code for the example, but does not yet include the data.

You can then build and append the embedded file system:

```
cd fs
export TARGET_DIR=../target/mips-nintendo64-none/release/
cat index.txt | xargs cat > ${TARGET_DIR}/fs.bin
cat ${TARGET_DIR}/slide-tool.z64 ${TARGET_DIR}/fs.bin > ${TARGET_DIR}/slide-tool-final.z64
rs64romtool chksum ${TARGET_DIR}/slide-tool-final.z64 ${TARGET_DIR}/slide-tool-final.z64
cd ..
```

The final ROM will be located at `target/mips-nintendo64-none/release/slide-tool-final.z64`

### Replacing Graphics

The directory [fs](./fs) includes graphics that are appended to the ROM. There are two .png files, which are the original images, and two .bin files, which are the same images converted to RGBA5551 format. This is the format used for textures on the N64.

To replace those .bin files with your own images, use the texture-converter tool included in this repo [here](../../tools/texture-converter/). The README file for the texture-converter project includes details for how to build the program and how to use it.

The list of .bin files that will be included in the final build are in `index.txt`. Update this list if you'd like to add or remove images.
