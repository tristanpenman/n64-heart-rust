[![License: MIT](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

# N64 💛 Rust

This repo contains content prepared for a talk at the [Melbourne Rust Meetup](https://www.meetup.com/rust-melbourne/), on the state of Rust on the Nintendo 64. The motivation for this talk comes from my initial exploration of the N64 Homebrew scene.

While there was a wealth of information available for the C programming language, I found it difficult to get a handle on the state of Rust on Nintendo 64. My questions included:

* What libraries are available, and how safe/mature are they for non-trivial projects?
* Which tools should I use for handling assets?
* Is it possible to use 3D graphics?

This repo is an attempt to collect that information in one place, and to update the content from my talk with the latest developments.

> [!NOTE]
> This README has been considerably revised since I first prepared content for the talk. If you would like to see the original slides, these can be found in [slides.pdf](./slides.pdf).

### Contents

- [Hardware](#hardware)
- [Homebrew Scene](#homebrew-scene)
- [Libraries and SDKs](#libraries-and-sdks)
- [Basics](#basics)
  - [Cross-Compilation](#cross-compilation)
  - [Bootstrap Code](#bootstrap-code)
  - [Barebones Development](#barebones-development)
  - [ROM Formats](#rom-formats)
- [Examples](#examples)
  - [Old School C](#01-old-school-c)
  - [Calling Rust from C](#02-calling-rust-from-c)
  - [Booting Rust](#03-booting-rust)
  - [Peripheral Access](#04-peripheral-access)
  - [Slide Tool](#05-slide-tool)
- [Tools](#tools)
- [References](#references)
- [Future](#future)

## Hardware

Let's start by revisiting the N64 hardware specifications. First released in 1996, the N64 boasted some impressive specs for the time...

* The main processor is a **64-bit NEC VR4300**, with 24KB of L1 cache. This is a MIPS R4300i-based CPU, running at 93.75 MHz. It uses the [MIPS Instruction Set Architecture](https://en.wikipedia.org/wiki/MIPS_architecture), similar to another [popular console](https://en.wikipedia.org/wiki/PlayStation_(console)) of that era.
* This is augmented with the **Reality Coprocessor (RCP)**, running at 62.5 MHz. This processor consists of two main components, the Reality Signal Processor (RSP), and the Reality Display Processor (RDP). The RSP can be used to process both graphics and audio data.
* The system includes **4.5MB of Rambus RDRAM**. 4MB of this is visible to the CPU, with the remaining 0.5MB used by the RCP for tasks such as anti-aliasing and Z-buffering. Expandable to 9MB (8MB CPU-accessible).

The RCP is an early example of a highly programmable GPU. It offers flexible graphics and audio processing capabilities, via programmable microcode.

## Homebrew Scene

The Nintendo 64 homebrew scene is buzzing right now. There's a very active Discord community, with yearly homebrew competitions. It used to be common for homebrew projects to use Nintendo's proprietary SDK (Libultra and NuSystem). These days, many homebrew developers choose to use [libdragon](https://github.com/DragonMinded/libdragon), an alternative SDK that has been developed from the ground up to be free and open source.

All of this is in large part thanks to the existence of flash carts, which make it possible to run homemade ROMs on physical hardware, without specialised SGI hardware. Flash carts support loading games via SD card or USB, with the USB interface also being available for debugging purposes.

## Libraries and SDKs

Programming the Nintendo 64 without software support (i.e. libraries) would be a daunting task. Unlike earlier consoles, the parallelism and complexity of the RCP means that each Nintendo 64 game effectively contains its own mini Operating System (OS).

The N64 OS is responsible for memory management, task scheduling, Direct Memory Access (DMA), and other low-level operations.

### Libultra

**Libultra** (part of the official Nintendo 64 SDK) is a library that includes functions for managing memory, scheduling tasks, accessing controllers and other input devices, and interfacing with the console's graphics and audio subsystems.

While not strictly necessary for Nintendo 64 development, Libultra handles some of the most basic requirements of any non-trivial ROM, and is better thought of as a minimal OS for the console.

We'll see shortly that it is possible to bypass Libultra entirely, and make a functioning N64 ROM using pure Rust code.

### NuSystem

For many projects, even Libultra would be considered unnecessarily low-level. The official N64 SDK also included a library called **NuSystem** (sometimes shortened to NuSys). This was designed to be easy to use than Libultra, and abstracts away many of the low-level details of programming the N64. This allows developers to focus on higher-level concerns, such as graphics and gameplay.

Even if you choose not to use NuSys for your own projects, it can still serve as a good reference, and the official SDK includes a number of examples for major features of the console.

### Libdragon

The third option is open source!

**Libdragon** is a fully open-source SDK for the Nintendo 64. It aims to provide a complete N64 development environment, while also providing programmers with modern development and debugging tools.

Libdragon was initially focused on 2D development, but significant progress has been made on 3D support, with the latest development ([unstable](https://github.com/DragonMinded/libdragon/wiki/Unstable-branch)) branch including an OpenGL 1.1 port, with full T&L support. It also provides a very modern debugging interface.

## Basics

Before we move on to examples, lets see what it takes to compile Rust for the MIPS processor in the N64. To do this, we'll rely on a technique called **Cross-Compilation**.

We'll also need to generate a ROM file that has the necessary format to be recognised by the console. The ROM must have an appropriate header, and it must contain **Bootstrap Code** that will be mapped to a specific location in memory.

### Cross-Compilation

Cross-Compilation is a technique that allows us to compile code for a system (the _target architecture_) that may be entirely different from the system that the compiler is running on (the _host architecture_). In this case, our target architecture is MIPS III, and the host will probably be AMD64 (Intel-based) or ARM64 (e.g. ARM-based Mac).

The N64's VR4300 CPU is based on the MIPS III instruction set, which happens to be an instruction set supported by `rustc`. The most straight-forward way to compile Rust to MIPS is to configure `cargo` using a JSON file that describes the target architecture and constraints on the output.

This is what the architecture file for example 2 looks like:

```
{
  "arch": "mips",
  "cpu": "mips3",
  "data-layout": "E-m:m-p:32:32-i8:8:32-i16:16:32-i64:64-n32:64-S64",
  "disable-redzone": true,
  "env": "unknown",
  "executables": true,
  "features": "+mips3,+noabicalls,+gp64,+fpxx,+nooddspreg",
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "llvm-abiname": "o32",
  "llvm-target": "mips-unknown-elf",
  "max-atomic-width": 32,
  "os": "none",
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-c-int-width": "32",
  "target-endian": "big",
  "target-family": "unix",
  "target-pointer-width": "32",
  "vendor": "nintendo64"
}

```

This is a nice mix of obvious and obscure. Some key details are the architecture and CPU (`"arch": "mips"`, `"cpu": "mips3"`) and the linker (`"linker": "rust-ldd"`).

While almost everything here is _important_, these two options highlight that we want to compile a MIPS object file, and that we will be linking it with our other object files.

#### ABI Compatibility

Another detail is in the line `"llvm-abiname": "o32"`, which specifies which Application Binary Interface (ABI) we want. See [here](https://techpubs.jurassic.nl/library/manuals/2000/007-2816-004/sgi_html/ch01.html) for a detailed write up of what an ABI is, and why it matters for the Nintendo 64.

For example 2, in which we link against object files generated by gcc, we want the `o32` ABI. For later examples, we'll use the `n32` ABI, which aims for slightly better performance.

### Bootstrap Code

Every N64 ROM must contain a block of bootstrap code, known as the IPL3 (Initial Program Loader 3). Unlike many cartridge based consoles, the N64 does not actually directly read from instructions and data from the cartridge. Instead, the console copies the first 4096 bytes of the ROM into memory, before executing it. This 4096 block of code is responsible for initialising various components, and copying the next block of code to be executed into main memory.

The original IPL3 is proprietary code from Nintendo, and was tied to the [CIC-NUS](https://n64brew.dev/wiki/CIC-NUS) chip installed the cartridge, which allowed Nintendo to enforce copyright protection. To build a ROM that will successfully run on real hardware, we must supply our own IPL3 binary. This was traditionally extracted from commercial ROMs.

#### Libdragon IPL

An alternative now is to use Libdragon's custom IPL, which is available on Libdragon's [preview branch](https://github.com/DragonMinded/libdragon/wiki/Preview-branch). This boots ROMs up to 5x faster and allows for compressed game code (using Libdragon's compression library).

> [!NOTE]
> that the examples in this repo assume that a commercial IPL3 binary is used.

#### Calling `main()`

It's worth noting that the examples in this repo include a small chunk of code, written in MIPS assembly. This code is responsible for performing some minimal setup before jumping to the `main()` function that is defined in Rust (or C, in the case of example 1).

### ROM Formats

One more detail worth mentioning is that N64 ROMs can come in one of three formats.

* Little Endian (.n64)
* Byte Swapped (.v64)
* Big Endian (.z64)

Although it is trivial to convert between these formats, which one is used will typically depend on how the ROM was produced. The key difference is how bytes are ordered within the ROM file.

The examples in this repo will produce .z64 files, using the native big-endian layout of the Nintendo 64.

## Examples

Time to look at the examples!

### 01: Old School C

When working with something like the Nintendo 64, it helps to a frame of reference. As a starting point,  provides the basic structure for a game written in C. It can be compiled with both legacy and modern SDKs, and includes minimal 3D graphics.

> [Source code](./examples/01-old-school-c/)

### 02: Calling Rust from C

The next example takes a small step towards using Rust, by calling Rust from C. This would allow you to write your game logic in Rust, but continue to rely on C to access SDK functionality.

> [Source code](./examples/02-calling-rust-from-c/)

### 03: Booting Rust

_Now things get serious!_ This example shows how you can build an entire ROM using Rust, plus a little bit of MIPS assembly to bootstrap the system. This example completely bypasses the Nintendo 64 SDK, interfacing with the hardware directly.

> [Source code](./examples/03-booting-rust/)

### 04. Peripheral Access

In the previous example, we used the low-level Video Interface (VI) to interact with the N64 hardware. This example shows how we can use a [Peripheral Access Crate](https://docs.rust-embedded.org/discovery/microbit/04-meet-your-hardware/terminology.html#peripheral-access-crate-pac) to better abstract access to memory-mapped registers.

> [Source code](./examples/04-peripheral-access)

### 05. Slide Tool

The next example shows how Rust can be used to display simple text and full-screen images. This makes use of memory-mapped I/O to interface directly with the framebuffer.

> [Source code](./examples/05-slide-tool)

## Tools

This section covers tools that have been included in this repo.

### Texture Converter

The first of these is a small Rust program for converting images from PNG to RGBA5551 format, appropriate for loading into textures and directly into the N64 framebuffer memory. This can be used to convert images for use with [Example 04](./examples/04-graphics-in-rust).

See the README in that directory contains instructions to build and use the texture converter tool.

> [Source code](tools/texture-converter/)

## References

Various notes about Nintendo 64 architecture and development can be found in the [notes](./notes) directory.

### Websites

**General**

* [Awesome N64 Development](https://n64.dev/)
  * Huge list of useful resources, including many of those listed below
* [N64 Hardware Architecture on YouTube](https://www.youtube.com/watch?v=U2uuH_ICRks)
  * Interesting video on the intricacies of emulation
* [Nintendo 64 Part 1: Land of Pain](https://www.moria.us/blog/2020/10/n64-part1-land-of-pain)
  * First post in an amazing series about developing for the Nintendo 64
  * Goes into great detail about MIPS, linking N64 binaries, handling assets, etc.

**Emulators**

* [ares](https://ares-emu.net/)
  * Cross-platform, open source, multi-system emulator, focusing on accuracy and preservation
* [cen64](https://github.com/n64dev/cen64)
  * Cycle-accurate N64 emulator, good for development
* [Project64](https://github.com/project64/project64)
  * N64 emulator for Windows with debugging support
* [sixtyforce](https://sixtyforce.com/)
  * Propietary macOS-based emulator, convenient for quick testing on macOS

**Discord**

* [N64brew Discord](https://discord.gg/WqFgNWf)
  * Very active for N64 development discussions

**Libdragon**

* [Libdragon](https://github.com/DragonMinded/libdragon)
  * Open-source SDK for Nintendo 64
  * `trunk` is stable code
    * Not always supported by emalators, due to issues with high level emulation
  * `unstable` contains many new features such as OpenGL 1.1 support
    * Not as well-supported by high level emulators at this stage
    * `spritemap` example at least runs on cen64 and real hardware, so that makes it possible to make a playable game

**Rust-specific**

* [nust64](https://github.com/rust-n64/nust64)
  * A tool for building Rust projects into N64 roms. Intended as a Cargo runner, but may also be used as a library.
  * Used for example 3 and 4
* [n64-pac](https://github.com/rust-n64/n64-pac)
  * Low-level abstraction (aka a Peripheral Access Crate) over the CPU and memory-mapped registers available on the Nintendo 64 console
* [n64-slides-apr](https://github.com/monocasa/n64-slides-apr)
  * Minimal Rust application that presents slides on N64
    * Most slides contain basic text
    * Some slides contain full images
  * Dependencies:
    * [rs64-rt](https://github.com/monocasa/rs64-rt)
    * [rs64-periph](https://github.com/monocasa/rs64-periph)
  * Makefile builds ELF, then extras object using `mips-unknown-elf-objcopy`
    * fs.bin for assets
    * combined with IPL boot code using [rs64romtool](https://github.com/monocasa/rs64romtool)
* [n64-systemtest](https://github.com/lemmy-64/n64-systemtest)
  * A comprehensive system test written in Rust, built using [nust64](https://github.com/rust-n64/nust64)
  * Works on real hardware

## Future

See the [PLAN.md](PLAN.md) for a list of potential improvements to this repo.

When time allows, I hope to add the following examples to this collection:

6. Libdragon in C - Incomplete
7. Libdragon in Rust - Not started
8. More Graphics in Rust - Not started

## License

The examples in this repo are licensed under the MIT License.

See the [LICENSE](./LICENSE) file for more information.
