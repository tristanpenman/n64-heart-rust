# Example 1 - Old School C

This example implements a simple spinning 3D box. It is implemented in two 'stages', to highlight the difference between boilerplate and graphics rendering code:

* [`stage0.c`](box/src/stage0.c) demonstrates basic graphics setup, but does not render any content
* [`stage1.c`](box/src/stage1.c) extends this to render a partial box, which is missing two ends so that it appears hollow

## Build

This example can be built using CrashOverride95's [Modern SDK](https://crashoveride95.github.io/n64hbrew/modernsdk/index.html), by running `make`. The output of the build will be written to `box.z64`.

By default, `stage1` will loaded at startup. You can modify [main.c](src/main.c) to change this:

    // #define STAGE 1
    #define STAGE 0

You will then need to run `make` to rebuild `box.z64`.
