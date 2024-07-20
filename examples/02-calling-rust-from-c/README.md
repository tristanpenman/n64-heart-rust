# Example 2 - Calling Rust from C

One way we can include Rust code in a Nintendo 64 ROM is to call Rust from C. Doing this allows us to compile some of our code as a Rust library, and invoke that from C, where we need it. This is not necessarily trivial, as we need to combine the output from two different toolchains. We have our GCC based toolchain for C, and our LLVM based toolchain for Rust.

## Prerequisites

This example will require the following to be installed:

* Modern SDK
* Rust Nightly (2022-06-21)

### Toolchain

The version of Rust Nightly is constrained by the [rust-toolchain.toml](./rust-toolchain.toml) file, as later versions currently crash while compiling for the MIPS `o32` ABI, which is required for linking with the GCC-generated object files. The presence of the `rust-toolchain.toml` file should ensure that the correct version of Rust is installed automatically.

## Build

Just run `make`. The final ROM will be written to `build/calling-rust-from-c.z64`.

When building for the first time, you should see output that looks similar to this:

```
Compiling: src/gfx.c -> build/src/gfx.o
Compiling: src/main.c -> build/src/main.o
Compiling: src/stage0.c -> build/src/stage0.o
Compiling: src/stage1.c -> build/src/stage1.o
Assembling: asm/entry.s -> build/asm/entry.o
Assembling: asm/rom_header.s -> build/asm/rom_header.o
Cargo: src/lib.rs -> target/mips-nintendo64-elf/debug/libminimal.a
info: syncing channel updates for 'nightly-2022-06-21-x86_64-unknown-linux-gnu'
info: latest update on 2022-06-21, rust version 1.63.0-nightly (5750a6aa2 2022-06-20)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-src'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
...
SNIP!
...
Preprocessing linker script: calling-rust-from-c.ld -> build/calling-rust-from-c.ld
Linking ELF file:  build/calling-rust-from-c.elf
Building ROM: build/calling-rust-from-c.elf -> build/calling-rust-from-c.z64
Masking build/calling-rust-from-c.z64.
Padding to 1052672 bytes
Generate checksum: 0x8E65251E956F8275
```

You can see here that the Rust toolchain is being installed automatically.

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

In order to compile Rust for the N64, we have to tell `rustc` about the target architecture. This is all contained in the file [mips-nintendo64-elf.json](./mips-nintendo64-elf.json). This file is a bit intimidating, but the keys points are that we're compiling for the MIPS III Instruction Set, which is almost identical to that used by the VR4300 in the Nintendo 64.

We also use the MIPS `o32` ABI, which specifies details such as calling conventions. See [here](https://techpubs.jurassic.nl/library/manuals/2000/007-2816-004/sgi_html/ch01.html) for a comparison of different ABIs supported by the MIPS architecture.

### Linking

Our [Cargo.toml](./Cargo.toml) file is pretty minimal, and just says that we're building a static library. The output will be a `.a` file, in ELF format, that will be linked into our final ROM. The name of the library will be `libminimal`.

Linking is specified (in great detail) in `calling-rust-from-c.ld`. This file is almost unchanged from [old-school-c.ld](../01-old-school-c/old-school-c.ld) in example 1, except that we include the relevant sections from `libminimal.a`.

## Acknowledgements

This example was originally based on work published in the [N64-Rust-Sample](https://github.com/Mr-Pnut/N64-Rust-Sample) repo on GitHub. It has since been updated to use `cargo` directly, instead of relying on `xargo`, and now uses the [Modern SDK](https://crashoveride95.github.io/modernsdk/index.html).
