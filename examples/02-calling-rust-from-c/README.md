# Example 2 - Calling Rust from C

One way we can include Rust code in a Nintendo 64 ROM is to call Rust from C. Doing this allows us to compile some of our code as a Rust library, and invoke that from C, where we need it.

**Warning**: This example depends on [Xargo](https://github.com/japaric/xargo), which has been in 'maintenance mode' for several years now.

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

You'll also need to install Xargo:

```
cargo install xargo
```

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
