# N64 💛 Rust

This repo contains source code for a selection of simple Nintendo 64 ROMs. These examples demonstrate how Rust can be used to program the Nintendo 64.

## Examples

1. [Old School C](#old-school-c)
2. [Calling Rust from C](#calling-rust-from-c)
3. [Booting from Rust](#booting-from-rust)
4. [Slide Tool](#slide-tool)
5. [Libdragon in C](#libdragon-in-c)
6. [Libdragon in Rust](#libdragon-in-rust)

### Old School C

When working with something like the Nintendo 64, it helps to a frame of reference. As a starting point, [01-old-school-c](./examples/01-old-school-c/) provides the basic structure for a game written in C. It can be compiled with both legacy and modern SDKs, and includes minimal 3D graphics.

### Calling Rust from C

The next example, [02-calling-rust-from-c](./examples/02-calling-rust-from-c/), takes a small step towards using Rust, by calling Rust from C. This would allow you to write your game logic in Rust, but continue to rely on C to access SDK functionality.

This example is based on MrPeanut's [N64-Rust-Sample](https://github.com/Mr-Pnut/N64-Rust-Sample) repo.

### Booting from Rust

Going a step further, [03-booting-rust](./examples/03-booting-rust/) looks at how we can build an entire ROM using Rust and Cargo. This completely bypasses the Nintendo 64 SDK, and interfaces with the hardware directly. This is essentially OS development on the N64.

This example is based on Jay Oster's [rrt0](https://github.com/rust-console/rrt0) repo.

### Slide Tool

The [04-slide-tool](./examples/04-slide-tool) shows how Rust can be used to make a simple slide presentation tool.

This example is an adaptation of Tristan Miller's [n64-slides-apr](https://github.com/monocasa/n64-slides-apr), [rs64-periph](https://github.com/monocasa/rs64-periph) and [rs64-rt](https://github.com/monocasa/rs64-rt) repos, incorporating minor changes from [rrt0](https://github.com/rust-console/rrt0).

### Libdragon in C

This is another example that serves as a reference point. This time we use [libdragon](https://github.com/DragonMinded/libdragon), an open source alternative to the official SDK. [05-libdragon-in-c](./examples/05-libdragon-in-c/) provides a small example that shows how Libdragon works in C.

### Libdragon in Rust

**NOTE: This example is not complete.**

In the final example, [06-libdragon-in-rust](./examples/06-libdragon-in-rust/), we look at how to work with Libdragon from Rust, taking advantage of some publically available Rust bindings.

## Notes

Various notes about Nintendo 64 architecture and development can be found in the [notes](./notes) directory.

## References

**Websites**

* Awesome N64 Development
  * https://n64.dev/

**Emulators**

* cen64
  * https://github.com/n64dev/cen64
  * Cycle-accurate N64 emulator, recommended for development
* Project64
  * https://github.com/project64/project64
  * N64 emulator for Windows
  * Excellent debugging support

**Discord**

* https://discord.gg/WqFgNWf
  * N64brew Discord, seems to be the most active for N64 development discussions

**Interesting N64-related repositories on GitHub**

* https://github.com/DragonMinded/libdragon
  * Libdragon is an open-source SDK for Nintendo 64
  * `trunk` is stable code
    * `spritemap` example at least runs on cen64 and real hardware, so that makes it possible to make a playable game
  * `unstable` contains many new features such as OpenGL 1.1 support
    * Not well-supported by emulators at this stage

**Interesting Rust-specific repositories on GitHub**

* https://github.com/monocasa/n64-slides-apr
  * Minimal Rust application that presents slides on N64
    * Most slides contain basic text
    * Some slides contain full images
  * Dependencies:
    * rs64-rt (https://github.com/monocasa/rs64-rt)
    * rs64-periph (https://github.com/monocasa/rs64-periph)
    * panic-halt
    * volatile
  * Makefile builds elf, then extras object using `mips-unknown-elf-objcopy`
    * fs.bin for assets (?)
    * combined with IPL boot code using `rs64romtool`
* https://github.com/lemmy-64/n64-systemtest
  * A comprehensive system test written in Rust
    * Built using nust64 (https://github.com/rust-n64/nust64)
    * Mentions cargo-n64 in the README, despite using nust64... what is the lineage there?
  * Works on real hardware
    * Also tested on cen64, but does not work there
* https://github.com/rust-n64/nust64
  * intended as a Cargo runner, but may also be used as a library
  * Not sure what the advantage of this over cargo-n64 is...
* https://github.com/DagothBob/libdragon-bindings
  * Rust bindings and interface to the N64-dev library LibDragon
  * Two years old at this point, and can't seem to find any examples online...
* https://github.com/rust-console/cargo-n64
  * A cargo subcommand to build Nintendo 64 ROMs in Rust
* https://github.com/rust-console/rrt0

## License

The examples in this repo are licensed under the MIT License.

See the LICENSE file for more information.
