# N64 💛 Rust

This repo will contain source code for a simple N64 ROM, built using Rust.

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
