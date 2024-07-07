# Example 3 - Booting Rust

This example shows how we can build an entire ROM using Rust and Cargo.

## Acknowledgements

This example is based on code that has been adapted from other repos:

* [rrt0](https://github.com/rust-console/rrt0) - A simple cross-platform runtime / startup for Rust on embedded devices (by [Jay Oster](https://github.com/parasyte))
* [n64-project-template](https://github.com/rust-n64/n64-project-template) - An N64 Project Template for Rust, based on nust64 (by [Luke Stadem](https://github.com/bigbass1997))

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

This will produce a complete N64 ROM, located at `target/mips-nintendo64-none/release/booting-rust.z64`.

## Details

Unlike previous examples, there is no SDK used here. The only Nintendo-derived code is the IPL3 (boot code) that we take from SDK, or extract from a ROM.

The entry point for our ROM is defined in [entrypoint.s](./rrt0/src/platforms/n64/entrypoint.s). This is written in MIPS assembly language.

Some very basic VI (Video Interface) definitions are included in the [n64lib](./n64lib/) library. This includes code to make use of a font embedded in the IPL3 binary.

## References

To learn more about the IPL:

* [What is the Nintendo 64 IPL3 and how could it be created by Rust developers?](https://retrocomputing.stackexchange.com/questions/14189/what-is-the-nintendo-64-ipl3-and-how-could-it-be-created-by-rust-developers) - A brief discussion of what would be required use to Rust to write boot code for the Nintendo 64
