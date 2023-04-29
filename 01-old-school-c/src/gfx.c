
#include <nusys.h>

#include "gfx.h"

#define MAX_GFX_TASKS 2

GfxTask gfx_tasks[MAX_GFX_TASKS];
Gfx* gfx_list_ptr;

int current_task = -1;

// Conversion from (-1,-1,-1)-(1,1,1), with a 2-bit decimal suffix
static Vp vp = {
    SCREEN_WD * 2, SCREEN_HT * 2, G_MAXZ / 2, 0,  // Scale
    SCREEN_WD * 2, SCREEN_HT * 2, G_MAXZ / 2, 0,  // Translate
};

// Display list to initialise RDP
static Gfx setup_rdpstate[] = {
    gsDPSetRenderMode(G_RM_OPA_SURF, G_RM_OPA_SURF2),
    gsDPSetCombineMode(G_CC_SHADE, G_CC_SHADE),
    gsDPSetScissor(G_SC_NON_INTERLACE, 0,0, SCREEN_WD, SCREEN_HT),
    gsDPSetColorDither(G_CD_BAYER),
    gsSPEndDisplayList()
};

// Display list to initialise RSP
static Gfx setup_rspstate[] = {
    gsSPViewport(&vp),
    gsSPClearGeometryMode(0xFFFFFFFF),
    gsSPSetGeometryMode(G_ZBUFFER | G_SHADE | G_SHADING_SMOOTH | G_CULL_BACK),
    gsSPTexture(0, 0, 0, 0, G_OFF),
    gsSPEndDisplayList()
};

void gfx_rcp_init(void)
{
    gSPSegment(gfx_list_ptr++, 0, 0x0);
    gSPDisplayList(gfx_list_ptr++, OS_K0_TO_PHYSICAL(setup_rspstate))
    gSPDisplayList(gfx_list_ptr++, OS_K0_TO_PHYSICAL(setup_rdpstate));
}

void gfx_clear_cfb(void)
{
    // Setting addresses of the frame buffer/Z-buffer and clear them using
    // nuGfxZBuffer (the address of the Z-buffer) and nuGfxCfb_ptr (the address
    // of the frame buffer) which are global variables of NuSYSTEM

    // Clear the Z-buffer
    gDPSetDepthImage(gfx_list_ptr++, OS_K0_TO_PHYSICAL(nuGfxZBuffer));
    gDPSetCycleType(gfx_list_ptr++, G_CYC_FILL);
    gDPSetColorImage(gfx_list_ptr++, G_IM_FMT_RGBA, G_IM_SIZ_16b,SCREEN_WD, OS_K0_TO_PHYSICAL(nuGfxZBuffer));
    gDPSetFillColor(gfx_list_ptr++, (GPACK_ZDZ(G_MAXFBZ,0) << 16 | GPACK_ZDZ(G_MAXFBZ,0)));
    gDPFillRectangle(gfx_list_ptr++, 0, 0, SCREEN_WD-1, SCREEN_HT-1);
    gDPPipeSync(gfx_list_ptr++);

    // Clear the frame buffer
    gDPSetColorImage(gfx_list_ptr++, G_IM_FMT_RGBA, G_IM_SIZ_16b, SCREEN_WD, osVirtualToPhysical(nuGfxCfb_ptr));
    gDPSetFillColor(gfx_list_ptr++, (GPACK_RGBA5551(0, 0, 0, 1) << 16 | GPACK_RGBA5551(0, 0, 0, 1)));
    gDPFillRectangle(gfx_list_ptr++, 0, 0, SCREEN_WD-1, SCREEN_HT-1);
    gDPPipeSync(gfx_list_ptr++);
}

GfxTask* gfx_new_task(void)
{
    GfxTask *next_task;

    // switch the current graphics task
    current_task = (current_task + 1) % MAX_GFX_TASKS;
    next_task = &gfx_tasks[current_task];
    gfx_list_ptr = &next_task->display_list[0];
    return next_task;
}
