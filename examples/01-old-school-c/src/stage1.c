#include <assert.h>
#include <nusys.h>

#include "gfx.h"
#include "stage1.h"
#include "vec.h"

#define FOVY 45
#define ASPECT (f32)SCREEN_WD/(f32)SCREEN_HT
#define NEAR_PLANE 10
#define FAR_PLANE 1000

static struct Vec3d camera_pos = {0, -200.0f, 200.0f};
static struct Vec3d camera_target = {0.0f, 0.0f, 0.0f};
static struct Vec3d camera_up = {0.0f, 1.0f, 0.0f};

float box_rotation_roll = 0;
float box_rotation_pitch = 0;

// A static array of model vertex data.
// This include the position (x,y,z), texture U,V coords (called S,T in the SDK)
// and vertex color values in r,g,b,a form.
// As this data will be read by the RCP via direct memory access, which is
// required to be 16-byte aligned, it's a good idea to annotate it with the GCC
// attribute `__attribute__((aligned (16)))`, to force it to be 16-byte aligned.
Vtx square_verts[] __attribute__((aligned (16))) = {
  //  x,   y,  z, flag, S, T,    r,    g,    b,    a
  // front
  { -64,  64, 64,    0, 0, 0, 0x00, 0xff, 0x00, 0xff  },
  {  64,  64, 64,    0, 0, 0, 0x00, 0x00, 0x00, 0xff  },
  {  64, -64, 64,    0, 0, 0, 0x00, 0x00, 0xff, 0xff  },
  { -64, -64, 64,    0, 0, 0, 0xff, 0x00, 0x00, 0xff  },
  // back
  { -64,  64, -64,   0, 0, 0, 0x00, 0xff, 0x00, 0xff  },
  {  64,  64, -64,   0, 0, 0, 0x00, 0x00, 0x00, 0xff  },
  {  64, -64, -64,   0, 0, 0, 0x00, 0x00, 0xff, 0xff  },
  { -64, -64, -64,   0, 0, 0, 0xff, 0x00, 0x00, 0xff  },
};

void stage1_draw_box()
{
    // load vertex data for the triangles
    gSPVertex(gfx_list_ptr++, &(square_verts[0]), 8, 0);
    // depending on which graphical features, the RDP might need to spend 1 or 2
    // cycles to render a primitive, and we need to tell it which to do
    gDPSetCycleType(gfx_list_ptr++, G_CYC_1CYCLE);
    // use antialiasing, rendering an opaque surface
    gDPSetRenderMode(gfx_list_ptr++, G_RM_AA_ZB_OPA_SURF, G_RM_AA_ZB_OPA_SURF2);
    // reset any rendering flags set when drawing the previous primitive
    gSPClearGeometryMode(gfx_list_ptr++, 0xFFFFFFFF);
    // enable smooth (gourad) shading and z-buffering
    gSPSetGeometryMode(gfx_list_ptr++, G_SHADE | G_SHADING_SMOOTH | G_ZBUFFER);

    // actually draw the triangles, using the specified vertices (front)
    gSP2Triangles(gfx_list_ptr++, 0, 1, 2, 0, 0, 2, 3, 0);

    // back
    gSP2Triangles(gfx_list_ptr++, 5, 4, 7, 5, 5, 7, 6, 5);

    // right
    gSP2Triangles(gfx_list_ptr++, 6, 5, 1, 6, 6, 1, 2, 6);

    // left
    gSP2Triangles(gfx_list_ptr++, 0, 7, 3, 0, 0, 4, 7, 0);

    // Mark that we've finished sending commands for this particular primitive.
    // This is needed to prevent race conditions inside the rendering hardware in
    // the case that subsequent commands change rendering settings.
    gDPPipeSync(gfx_list_ptr++);
}

void stage1_draw()
{
    unsigned short persp_norm;
    GfxTask *gfx_task = gfx_new_task();

    // prepare the RCP for rendering a graphics task
    gfx_rcp_init();

    // clear the color framebuffer and Z-buffer, similar to glClear()
    gfx_clear_cfb();

    // initialize the projection matrix, similar to glPerspective() or glm::perspective()
    guPerspective(
        &gfx_task->projection,
        &persp_norm,
        FOVY,
        ASPECT,
        NEAR_PLANE,
        FAR_PLANE,
        1.0
    );

    // Our first actual displaylist command. This writes the command as a value at
    // the tail of the current display list, and we increment the display list
    // tail pointer, ready for the next command to be written.
    // As for what this command does... it's just required when using a perspective
    // projection. Try pasting 'gSPPerspNormalize' into google if you want more
    // explanation, as all the SDK documentation has been placed online by
    // hobbyists and is well indexed.
    gSPPerspNormalize(gfx_list_ptr++, persp_norm);

    // initialize the modelview matrix, similar to gluLookAt() or glm::lookAt()
    guLookAt(
        &gfx_task->modelview,
        camera_pos.x,
        camera_pos.y,
        camera_pos.z,
        camera_target.x,
        camera_target.y,
        camera_target.z,
        camera_up.x,
        camera_up.y,
        camera_up.z
    );

    // load the projection matrix into the matrix stack.
    // given the combination of G_MTX_flags we provide, effectively this means
    // "replace the projection matrix with this new matrix"
    gSPMatrix(
        gfx_list_ptr++,
        // we use the OS_K0_TO_PHYSICAL macro to convert the pointer to this matrix
        // into a 'physical' address as required by the RCP
        OS_K0_TO_PHYSICAL(&(gfx_task->projection)),
        // these flags tell the graphics microcode what to do with this matrix
        // documented here: http://n64devkit.square7.ch/tutorial/graphics/1/1_3.htm
        G_MTX_PROJECTION |  // using the projection matrix stack...
        G_MTX_LOAD |        // don't multiply matrix by previously-top matrix in stack
        G_MTX_NOPUSH        // don't push another matrix onto the stack before operation
    );

    gSPMatrix(
        gfx_list_ptr++,
        OS_K0_TO_PHYSICAL(&(gfx_task->modelview)),
        // similarly this combination means "replace the modelview matrix with this new matrix"
        G_MTX_MODELVIEW | G_MTX_NOPUSH | G_MTX_LOAD
    );

    // create a transformation matrix representing the position of the square
    guPosition(
        &gfx_task->object_transforms[0],
        box_rotation_roll,  // roll
        box_rotation_pitch, // pitch
        0.0f,               // heading
        1.0f,               // scale
        0, 0, 0             // position
    );

    // push relative transformation matrix
    gSPMatrix(gfx_list_ptr++,
        OS_K0_TO_PHYSICAL(&(gfx_task->object_transforms[0])),
        G_MTX_MODELVIEW |  // operating on the modelview matrix stack...
        G_MTX_PUSH |       // ...push another matrix onto the stack...
        G_MTX_MUL          // ...which is multipled by previously-top matrix (eg. a relative transformation)
    );

    stage1_draw_box();

    // pop the matrix that we added back off the stack, to move the drawing position
    // back to where it was before we rendered this object
    gSPPopMatrix(gfx_list_ptr++, G_MTX_MODELVIEW);

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

void stage1_update()
{
    box_rotation_roll = box_rotation_roll + 1.f;
    if (box_rotation_roll > 360.f) {
        box_rotation_roll = 0.f;
    }

    box_rotation_pitch = box_rotation_pitch + 0.5f;
    if (box_rotation_pitch > 360.f) {
        box_rotation_pitch = 0.f;
    }
}

void stage1_init()
{
    // ========================================================================
    //
    // TODO: Your scene initialisation code goes here
    //
    // ========================================================================
}

void stage1_loop(int pending_gfx)
{
    if (pending_gfx < 1) {
        stage1_draw();
    }

    stage1_update();
}
