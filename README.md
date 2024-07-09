[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

# N64 💛 Rust

This repo contains code for a talk given at the Melbourne Rust Meetup, on using Rust on the Nintendo 64. The examples demonstrate various ways that Rust can be used to program the Nintendo 64.

If you would like to see the slides from the talk, these can be found in [slides.pdf](./slides.pdf).

## Examples

1. [Old School C](#old-school-c)
2. [Calling Rust from C](#calling-rust-from-c)
3. [Booting from Rust](#booting-from-rust)
4. [Graphics in Rust](#graphics-in-rust)
5. [Libdragon in C](#libdragon-in-c)
6. [Libdragon in Rust](#libdragon-in-rust) - Incomplete
7. [More Graphics in Rust](#more-graphics-in-rust) - Incomplete

### Old School C

When working with something like the Nintendo 64, it helps to a frame of reference. As a starting point, [01-old-school-c](./examples/01-old-school-c/) provides the basic structure for a game written in C. It can be compiled with both legacy and modern SDKs, and includes minimal 3D graphics.

### Calling Rust from C

The next example, [02-calling-rust-from-c](./examples/02-calling-rust-from-c/), takes a small step towards using Rust, by calling Rust from C. This would allow you to write your game logic in Rust, but continue to rely on C to access SDK functionality.

### Booting from Rust

Going a step further, [03-booting-rust](./examples/03-booting-rust/) looks at how we can build an entire ROM using Rust and Cargo. This completely bypasses the Nintendo 64 SDK, and interfaces with the hardware directly. This is essentially OS development on the N64.

### Graphics in Rust

The [04-graphics-in-rust](./examples/04-graphics-in-rust) shows how Rust can be used to display simple text and full-screen images.

### Libdragon in C

This is another example that serves as a reference point. This time we use [libdragon](https://github.com/DragonMinded/libdragon), an open source alternative to the official SDK. [05-libdragon-in-c](./examples/05-libdragon-in-c/) provides a small example that shows how Libdragon works in C.

### Libdragon in Rust

**NOTE: This example is incomplete.**

The aim of the next example, [06-libdragon-in-rust](./examples/06-libdragon-in-rust/), is to show how we can use Libdragon from Rust, using Rust bindings to the Libdragon library.

### More Graphics In Rust

**NOTE: This example is incomplete.**

The aim of the final example is to show how Rust can be used for 3D graphics.

## Tools

1. [Texture Converter](#texture-converter)

### Texture Converter

This repo includes a small Rust program for converting images from PNG to RGBA5551 format, appropriate for loading into textures and directly into the N64 framebuffer memory. This can be used to convert images for use with [Example 04](./examples/04-graphics-in-rust).

The source code can be found in [tools/texture-converter](tools/texture-converter/).

See the README in that directory contains instructions to build and use the texture converter tool.

## Notes

Various notes about Nintendo 64 architecture and development can be found in the [notes](./notes) directory.

## References

**Websites**

* [Awesome N64 Development](https://n64.dev/)
  * Huge list of useful resources, including many of those listed below
* [N64 Hardware Architecture on YouTube](https://www.youtube.com/watch?v=U2uuH_ICRks)
  * Interesting video on the intricacies of emulation
* [Nintendo 64 Part 1: Land of Pain](https://www.moria.us/blog/2020/10/n64-part1-land-of-pain)
  * First post in an amazing series about developing for the Nintendo 64
  * Goes into great detail about MIPS, linking N64 binaries, handling assets, etc.

**Emulators**

* [cen64](https://github.com/n64dev/cen64)
  * Cycle-accurate N64 emulator, recommended for development
* [Project64](https://github.com/project64/project64)
  * N64 emulator for Windows
  * Excellent debugging support
* [sixtyforce](https://sixtyforce.com/)
  * Propietary macOS-based emulator
  * High performance and convenient for quick testing

**Discord**

* [N64brew Discord](https://discord.gg/WqFgNWf)
  * Very active for N64 development discussions

**Interesting N64-related repositories on GitHub**

* [Libdragon](https://github.com/DragonMinded/libdragon)
  * Libdragon is an open-source SDK for Nintendo 64
  * `trunk` is stable code
  * `unstable` contains many new features such as OpenGL 1.1 support
    * Not well-supported by emulators at this stage
  * Not always supported by emalators, due to issues with high level emulation
    * `spritemap` example at least runs on cen64 and real hardware, so that makes it possible to make a playable game
* [asterois64](https://github.com/tristanpenman/asteroids64)
  * This is my clone of Asteroids for the Nintendo 64, written in C
  * Uses official SDK
  * Compiles using legacy and modern toolchains

**Interesting Rust-specific repositories on GitHub**

* [n64-slides-apr](https://github.com/monocasa/n64-slides-apr)
  * Minimal Rust application that presents slides on N64
    * Most slides contain basic text
    * Some slides contain full images
  * Dependencies:
    * [rs64-rt](https://github.com/monocasa/rs64-rt)
    * [rs64-periph](https://github.com/monocasa/rs64-periph)
    * panic-halt
    * volatile
  * Makefile builds elf, then extras object using `mips-unknown-elf-objcopy`
    * fs.bin for assets
    * combined with IPL boot code using [rs64romtool](https://github.com/monocasa/rs64romtool)
* [n64-systemtest](https://github.com/lemmy-64/n64-systemtest)
  * A comprehensive system test written in Rust
    * Built using [nust64](https://github.com/rust-n64/nust64)
  * Works on real hardware
    * Also tested on cen64, but does not work there
* [cargo-n64](https://github.com/rust-console/cargo-n64)
  * A cargo subcommand to build Nintendo 64 ROMs in Rust
  * This is what I used for my examples
* [nust64](https://github.com/rust-n64/nust64)
  * Intended as a Cargo runner, but may also be used as a library
  * Not sure what the advantage of this over cargo-n64 is
* [libdragon-bindings](https://github.com/DagothBob/libdragon-bindings)
  * Rust bindings and interface to the N64-dev library LibDragon
  * Couldn't get it to compile
  * Two years old at this point, and can't seem to find any examples online...
* [rrt0](https://github.com/rust-console/rrt0)
  * Minimal Rust runtime, with a MIPS assembly entry point for N64

## License

The examples in this repo are licensed under the MIT License.

See the [LICENSE](./LICENSE) file for more information.
