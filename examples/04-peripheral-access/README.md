# Example 4 - Peripheral Access

This example shows how we can use a Peripheral Access Crate to abstract access to the CPU and memory-mapped registers on the N64.

## Acknowledgements

This example is based on code released by [Luke Stadem](https://github.com/bigbass1997):

* [n64-pac](https://github.com/bigbass1997/n64-pac) - A Peripheral Access Crate for the N64
* [n64-project-template](https://github.com/rust-n64/n64-project-template) - An N64 Project Template for Rust, based on nust64

## Prerequisites

This example is pure Rust, so there are no SDK dependencies. It does, however, require the installation of [nust64](https://github.com/rust-n64/nust64), an ELF binary to N64 ROM converter:

```
cargo install nust64
```

## IPL3

Building the ROM requires an IPL3 binary. Nintendo's propietary IPL3 binary can be found in the Nintendo 64 SDK. It can also be extracted from an official ROM.

Alternatively, you can use an open source IPL3 implementation such as [this one](https://github.com/PeterLemon/N64/tree/master/BOOTCODE) (by Peter Lemon). This has been designed to produce a binary that is byte-for-byte equivalent to the original IPL3 binary.

Depending on your choice of IPL3 binary, you may need to be update [.cargo/config.toml](.cargo/config.toml) with the necessary path.

## Build

To compile a release build using `cargo`, with an existing IPL3 binary blob:

```
cargo run --release
```

This will produce a complete N64 ROM, located at `target/mips-nintendo64-none/release/peripheral-access.z64`.
