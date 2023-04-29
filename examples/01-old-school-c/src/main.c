#include <nusys.h>

#include "stage0.h"
#include "stage1.h"

// use this to choose which stage to load on startup
#define STAGE 1

void mainproc()
{
    nuGfxInit();

#if STAGE == 0
    stage0_init();
    nuGfxFuncSet((NUGfxFunc)stage0_loop);
#elif STAGE == 1
    stage1_init();
    nuGfxFuncSet((NUGfxFunc)stage1_loop);
#endif

    nuGfxDisplayOn();

    while (1);
}
