#include <nusys.h>

#include "stage0.h"
#include "stage1.h"

int call_rust();

void mainproc()
{
    nuGfxInit();

    if (call_rust() == 0) {
      stage0_init();
      nuGfxFuncSet((NUGfxFunc)stage0_loop);
    } else {
      stage1_init();
      nuGfxFuncSet((NUGfxFunc)stage1_loop);
    }

    nuGfxDisplayOn();

    while (1);
}
