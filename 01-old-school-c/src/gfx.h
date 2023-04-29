#ifndef __GFX_H
#define __GFX_H

#include <nusys.h>

#define SCREEN_HT 240
#define SCREEN_WD 320

#define SCREEN_RATIO ((float)SCREEN_HT / (float)SCREEN_WD)

// The maximum length of the display list of one task
#define GFX_DISPLAY_LIST_LEN 2048
#define GFX_MAX_OBJECTS 10

// The projection-matrix structure
typedef struct {
    Mtx projection;
    Mtx modelview;

    Mtx object_transforms[GFX_MAX_OBJECTS];

    Gfx display_list[GFX_DISPLAY_LIST_LEN];
} GfxTask;

extern Gfx* gfx_list_ptr;

void gfx_rcp_init(void);
void gfx_clear_cfb(void);

GfxTask* gfx_new_task(void);

#endif
