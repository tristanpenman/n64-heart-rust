# Example 4 - Graphics in Rust

This example builds upon [03-booting-rust](../examples/03-booting-rust) by introducing some basic graphics and interactivity. We'll see that it's not too difficult to use the Nintendo 64 to display images and text.

## Acknowledgements

The code in this example is based on Tristan Miller's [n64-slide-apr](https://github.com/monocasa/n64-slides-apr). It turns out there are other Tristans in the world who want to see Rust on the Nintendo 64.

The original code used two other repos published by Miller:

* [rs64-periph](https://github.com/monocasa/rs64-periph) - Memory-mapped I/O (MMIO) definitions
* [rs64-rt](https://github.com/monocasa/rs64-rt) - Minimal startup / runtime

These required some minor modifications to compile, which are included here in [periph](./periph) and [rt](./rt).

## Prerequisites

Install cargo-n64:

```
cargo +nightly-2022-06-21 install --git https://github.com/rust-console/cargo-n64/ --rev 98f23023dcd2eae21484179309f58eb7ddd5bfd5
```

Install rs64-romtool (used to update checksum at the end):

```
cargo install rs64-romtool
```

## Build

A simple Makefile has been included in this project, meaning the easiest way to build the ROM is to run `make`.

This will ensure that the contents of [fs](./fs) are correctly appended to the ROM.
