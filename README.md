[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

# N64 💛 Rust

This repo contains content prepared for a talk at the [Melbourne Rust Meetup](https://www.meetup.com/rust-melbourne/), on the state of Rust on the Nintendo 64.

## Background

The motivation for this talk stems from my initial exploration into the N64 Homebrew scene. While there was a wealth of information available for the C programming language, I found it difficult to get a handle on the state of Rust on Nintendo 64. My questions included:

* What libraries are available, and how safe/mature are they for non-trivial projects?
* Which tools should I use for handling assets?
* Is it possible to use 3D graphics (yet)?

This repo is an attempt to collect that information in one place, and to update the content from my talk with the latest developments.

### Slides

This README has been considerably revised since I first prepared content for the talk. If you would like to see the original slides, these can be found in [slides.pdf](./slides.pdf).

### Code

For those looking for code, here is the list of examples that were covered in the talk, updated as necessary:

1. [Old School C](#old-school-c)
2. [Calling Rust from C](#calling-rust-from-c)
3. [Booting from Rust](#booting-from-rust)
4. [Graphics in Rust](#graphics-in-rust)
5. [Libdragon in C](#libdragon-in-c)
6. [Libdragon in Rust](#libdragon-in-rust) - Incomplete
7. [More Graphics in Rust](#more-graphics-in-rust) - Incomplete

These are described in more detail below.

## Hardware

Let's start by revisiting the N64 hardware specifications. First released in 1996, the N64 boasted some impressive specs for the time...

* The main processor is a **64-bit NEC VR4300**, with 24KB of L1 cache. This is a MIPS R4300i-based CPU, running at 93.75 MHz. Raw performance is 125 MIPS (million instructions per second).
* This is augmented with the **Reality Coprocessor (RCP)**, running at 62.5 MHz. This processor consists of two main components, the Reality Signal Processor (RSP), and the Reality Display Processor (RDP).
* The system includes **4.5MB of Rambus RDRAM**. 4MB of this is visible to the CPU, with the remaining 0.5MB used by the RCP for tasks such as anti-aliasing and Z-buffering. Expandable to 9MB (8MB CPU-accessible).

The RCP is an early example of a highly programmable GPU. It offers flexible graphics and audio processing capabilities, via programmable microcode.

## Homebrew Scene

The Nintendo 64 homebrew scene is buzzing right now. There's a very active Discord community, with yearly homebrew competitions. It used to be common for homebrew projects to use Nintendo's proprietary SDK (Libultra and NuSystem). These days, many homebrew developers choose to use [libdragon](https://github.com/DragonMinded/libdragon), an alternative SDK that has been developed from the ground up to be free and open source.

All of this is in large part thanks to the existence of flash carts, which make it possible to run homemade ROMs on physical hardware, without specialised SGI hardware. Flash carts support loading games via SD card or USB, with the USB interface also being available for debugging purposes.

## Libraries

Programming the Nintendo 64 without software support (i.e. libraries) is a daunting task. Unlike earlier consoles, the parallelism and complexity of the RCP means that each Nintendo 64 game effectively contains its own mini operating system.

### Libultra

Libultra (part of the official Nintendo 64 SDK) is a library that included functions for managing memory, scheduling tasks, accessing controllers and other input devices, and interfacing with the console's graphics and audio subsystems.

While not strictly necessary for Nintendo 64 development, Libultra handles some of the most basic requirements of any non-trivial ROM, and is better thought of as a minimal OS for the console.

We'll see shortly that it is possible to bypass Libultra entirely, and make a functioning N64 ROM using pure Rust code.

### NuSystem

For many projects, even Libultra would be considered unnecessarily low-level. The official N64 SDK also included a library called NuSystem. This was designed to be easy to use (easier than Libultra!). It abstracts away many of the low-level details of programming the N64, allowing developers to focus on high-level game development.

Even if you choose not to use NuSystem for your own projects, it is a good place to start for getting a grasp on N64 development, and the official SDK includes a number of examples for all the major features of the console.

### Libdragon

Libdragon is an open-source SDK for Nintendo 64. It aims for a complete N64 programming experience while providing programmers with modern approach to programming and debugging.

Libdragon was initially focused on 2D development, but significant progress has been made on 3D support, with the latest development ([unstable](https://github.com/DragonMinded/libdragon/wiki/Unstable-branch)) branch including an OpenGL 1.1 port, with full T&L support. It also provides a very modern debugging interface.

## Basics

Our first step is to compile Rust for the MIPS processor in the N64. The N64's VR4300 CPU is based on the MIPS III instruction set, which happens to be an instruction set supported by `rustc`. The most straight-forward way to do this is to use `xargo`, passing in a JSON file describing the target architecture.

TODO

## Examples

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

## Research

### Nust64

https://github.com/rust-n64/nust64
https://github.com/rust-n64/n64-pac

## License

The examples in this repo are licensed under the MIT License.

See the [LICENSE](./LICENSE) file for more information.
