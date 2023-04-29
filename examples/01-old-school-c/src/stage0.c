#include <assert.h>
#include <nusys.h>

#include "gfx.h"
#include "stage0.h"

void stage0_draw()
{
    unsigned short persp_norm;
    GfxTask *gfx_task = gfx_new_task();

    // prepare the RCP for rendering a graphics task
    gfx_rcp_init();

    // clear the color framebuffer and Z-buffer, similar to glClear()
    gfx_clear_cfb();

    // ========================================================================
    //
    // TODO: Your graphics code goes here. See stage1.c for an example.
    //
    // ========================================================================

    // mark the end of the display list
    gDPFullSync(gfx_list_ptr++);
    gSPEndDisplayList(gfx_list_ptr++);

    // assert that the display list isn't longer than the memory allocated for it,
    // otherwise we would have corrupted memory when writing it.
    // isn't unsafe memory access fun?
    // this could be made safer by instead asserting on the displaylist length
    // every time the pointer is advanced, but that would add some overhead.
    assert(gfx_list_ptr - gfx_task->display_list < GFX_DISPLAY_LIST_LEN);

    // create a graphics task to render this displaylist and send it to the RCP
    nuGfxTaskStart(
        gfx_task->display_list,
        (s32)(gfx_list_ptr - gfx_task->display_list) * sizeof (Gfx),
        NU_GFX_UCODE_F3DEX,  // load the 'F3DEX' version graphics microcode, which runs on the RCP to process this display list
        NU_SC_SWAPBUFFER     // tells NuSystem to immediately display the frame on screen after the RCP finishes rendering it
    );
}

void stage0_update()
{
    // ========================================================================
    //
    // TODO: Your scene update code goes here
    //
    // ========================================================================
}

void stage0_init()
{
    // ========================================================================
    //
    // TODO: Your scene initialisation code goes here
    //
    // ========================================================================
}

void stage0_loop(int pending_gfx)
{
    if (pending_gfx < 1) {
        stage0_draw();
    }

    stage0_update();
}
