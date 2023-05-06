# Example 2 - Calling Rust from C

The easiest way for us to integrate Rust code into a Nintendo 64 ROM is to call Rust from C. Doing this allows us to compile some of our code as a Rust library, and call to that from C.

## Acknowledgements

This example is based on work published in the [N64-Rust-Sample](https://github.com/Mr-Pnut/N64-Rust-Sample) repo on GitHub.

## Prerequisites

This example will require the following to be installed:

* Modern SDK
* Rust Nightly (2022-06-21)

Rust Nightly can be installed using `rustup`:

```
rustup toolchain install $(cat rust-toolchain)
rustup run $(cat rust-toolchain) -- rustup component add rust-src
```

The version of Rust Nightly is constrained by the [rust-toolchain](./rust-toolchain) file, as later versions currently crash while compiling for MIPS.

## Build

Just run `make`. The final ROM will be written to `build/calling-rust-from-c.z64`.

## Details

This example is based on [example 1](../01-old-school-c/). However, instead of choosing which stage to initialise using a `#define`, we call a function defined in a Rust library. This function is defined as:

```
#[no_mangle]
extern "C" fn call_rust() -> i32 {
  1
}
```

In C, it has the following function signature:

```
int call_rust();
```

Returning 0 from this function will select stage 0 (a blank screen), while returning 1 will select stage 1 (spinning box).

### Cross Compilation

In order to compile Rust for the N64, we have to tell `rustc` about the target architecture. This is all contained in the file [mips-n64-elf.json](./mips-n64-elf.json). This file is a bit intimidating, but the keys points are that we're compiling for the MIPS III Instruction Set, which is almost identical to that used by the VR4300 in the Nintendo 64.

### Linking

Our [Cargo.toml](./Cargo.toml) file is pretty minimal, and just says that we're building a static library. The output will be a `.a` file, in ELF format, that will be linked into our final ROM. The name of the library will be `libminimal`.

Linking is specified (in great detail) in `calling-rust-from-c.ld`. This file is almost unchanged from [old-school-c.ld](../01-old-school-c/old-school-c.ld) in example 1, except that we include the relevant sections from `libminimal.a`.

## Known Issues

Inconsistent build flags:

```
mips-n64-ld: warning: build/calling-rust-from-c.elf uses -mdouble-float (set by build/asm/rom_header.o), target/mips-n64-elf/debug/libminimal.a(minimal-60569bb7228f5707.1h9y3z07wpttnsvk.rcgu.o) uses -mgp32 -mfp64
```

Doesn't affect the stability of this example, but could become relevant if floating point operations are involved.
