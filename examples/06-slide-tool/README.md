# Example 6 - Slide Tool

This example builds upon [03-booting-rust](../examples/03-booting-rust) by introducing some basic interactivity. We'll see that it's not too difficult to use the Nintendo 64 to display a simple slide deck.

## Acknowledgements

The code in this example is based on Tristan Miller's [n64-slide-apr](https://github.com/monocasa/n64-slides-apr). It turns out there are other Tristans in the world who want to see Rust on the Nintendo 64.

The original code used two other repos published by Miller:

* [rs64-periph](https://github.com/monocasa/rs64-periph) - Memory-mapped I/O (MMIO) definitions
* [rs64-rt](https://github.com/monocasa/rs64-rt) - Minimal startup / runtime

These required some minor modifications to compile, which are included here in [periph](./periph) and [rt](./rt).

## Build

To build with an existing IPL3 binary blob:

    cargo n64 build --ipl3 <path-to-ipl3.bin> -- --package hello-ipl3font

To build using the IPL3 extracted from an existing ROM:

    cargo n64 build --ipl3-from-rom <path-to-rom.z64> -- --package hello-ipl3font
