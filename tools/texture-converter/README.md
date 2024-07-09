# Texture Converter

A small Rust program for converting PNG images to RGBA5551 format.

This can be used to produce images for use with [Example 04 - Graphics in Rust](../../examples/04-graphics-in-rust/).

> [!WARNING]
> This is currently a work in progress.

## Build

No surprises here:

    cargo build

This will produce an executable at `target/debug/texture-converter`.

Alternatively you can build a release executable:

    cargo build --release

## Usage

To convert a PNG file `<input>` to an RGBA5551 format file `<output>`:

    texture-converter <input> <output>

For example, if you are running `texture-converter` from the current directory, here is how you would convert slides for [Example 04](../../examples/04-graphics-in-rust/):

    ./target/debug/texture-converter \
      ../../examples/04-graphics-in-rust/fs/slide-01.png \
      ../../examples/04-graphics-in-rust/fs/slide-01.bin
