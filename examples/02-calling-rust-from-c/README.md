# Example 2 - Calling Rust from C

The easiest way for us to integrate Rust code into a Nintendo 64 ROM is to call Rust from C. Doing this allows us to compile some of our code as a Rust library, and call to that from C.

## Prerequisites

This example will require the following to be installed:

* Modern SDK
* Rust Nightly (2022-06-21)

Rust Nightly can be installed using `rustup`:

```
rustup toolchain install $(cat rust-toolchain)
rustup run $(cat rust-toolchain) -- rustup component add rust-src
```

## Build

Just run `make`. The final ROM will be written to `build/calling-rust-from-c.z64`.

## Known Issues

Inconsistent build flags:

```
mips-n64-ld: warning: build/calling-rust-from-c.elf uses -mdouble-float (set by build/asm/rom_header.o), target/mips-n64-elf/debug/libminimal.a(minimal-60569bb7228f5707.1h9y3z07wpttnsvk.rcgu.o) uses -mgp32 -mfp64
```

## Acknowledgements

This example is based on work published by [MrPeanut](https://github.com/Mr-Pnut) in the [N64-Rust-Sample](https://github.com/Mr-Pnut/N64-Rust-Sample) repo on GitHub.
