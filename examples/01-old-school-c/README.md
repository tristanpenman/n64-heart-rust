# Example 1 - Old School C

This example implements a simple spinning 3D box.

## Prerequisites

This example will require the following to be installed:

* Modern SDK

The Makefile included in this example is meant to be used with the CrashOverride95's [Modern SDK](https://crashoveride95.github.io/n64hbrew/modernsdk/index.html). Please follow the installation instructions there to install the SDK.

## Build

This example can be built using `make`.

The output of the build will be written to `build/old-school-c.z64`.

## Details

This example is implemented as two 'stages', to highlight the difference between boilerplate and graphics rendering code:

* [`stage0.c`](box/src/stage0.c) demonstrates basic OS and graphics setup, but does not render any content
* [`stage1.c`](box/src/stage1.c) extends this to render a 3D box, which is missing two ends so that it appears hollow

By default, `stage1` will loaded at startup. You can modify [main.c](src/main.c) to change this:

    // #define STAGE 1
    #define STAGE 0

You will then need to run `make` to re-build `old-school-c.z64`.

### Boilerplate

This example contains a lot of boilerplate, which is required to boot the Nintendo 64 into a useable state.

* [asm/entry.s](./asm/entry.s) - minimal code to jump to the `nuBoot` boot function
* [asm/rom_header.s](./asm/rom_header.s) - 64 byte ROM header
* [include/macros.inc](./include/macros.inc) - assembly macros used in `entry.s`

The code in these files is sufficient to transfer control to the `nuBoot` function (provided by NuSystem), which in turn calls `mainproc` (defined in [main.c](./src/main.c)).

### Linking

Nintendo 64 ROMS are essentially flat binaries. They begin with a 64 byte header, followed by 4KB of code that is responsible for loading everything beyond the first 4KB boundary. We can think of this as being like a boot sector in the MBR partitioning scheme.

The structure of the ROM is determined by a linker configuration file (in this case [old-school-c.ld](./old-school-c.ld)). The linker configuration file specifies code and data sections
