# Example 3 - Booting Rust

## Prerequisites

This example is pure Rust, so there are no SDK dependencies. It does, however, require the use of a custom `cargo` subcommand:

* [cargo-n64](https://github.com/rust-console/cargo-n64) - A `cargo` subcommand to build Nintendo 64 ROMs in Rust
* [N64 IPL3](https://retrocomputing.stackexchange.com/questions/14189/what-is-the-nintendo-64-ipl3-and-how-could-it-be-created-by-rust-developers) - boot code for the N64

You can install `cargo-n64` from crates.io:

    cargo install cargo-n64

The N64 IPL3 binary can be found in the SDK, or extracted from a ROM.

## Build

To build with an existing IPL3 binary blob:

    cargo n64 build --ipl3 <path-to-ipl3.bin> -- --package hello-ipl3font

To build using the IPL3 extracted from an existing ROM:

    cargo n64 build --ipl3-from-rom <path-to-rom.z64> -- --package hello-ipl3font

## Details

Unlike previous examples, there is no SDK being used here. The only Nintendo-derived code is the IPL3 (boot code) that we take from SDK, or extract from a ROM.

The entry point for our ROM is defined in [entrypoint.s](./rrt0/src/platforms/n64/entrypoint.s). This is written in MIPS assembly language.

Some very basic VI (Video Interface) functionality is defined in the [n64lib](./n64lib/) library. This includes code to make use of a font embedded in the IPL3 binary.

## Acknowledgements

This example is based on work published in the [rrt0](https://github.com/rust-console/rrt0) repo on GitHub.

