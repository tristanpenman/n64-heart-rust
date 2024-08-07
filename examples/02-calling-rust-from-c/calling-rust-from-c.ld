/*
  ROM spec file

  Main memory map

  0x80000000  exception vectors, ...
  0x80000400  zbuffer (size 320*240*2)
  0x80025c00  codesegment
       :
  0x8038F800  cfb 16b 3buffer (size 320*240*2*3)

  Copyright (C) 1997-1999, NINTENDO Co,Ltd.
*/

OUTPUT_ARCH (mips)

#include <nusys.h>

/* Starts a segment */
#define BEGIN_SEG(name, addr) \
    _##name##SegmentStart = ADDR(.name); \
    _##name##SegmentRomStart = __romPos; \
    .name addr : AT(__romPos)

/* Ends a segment */
#define END_SEG(name) \
    _##name##SegmentEnd = ADDR(.name) + SIZEOF(.name); \
    _##name##SegmentRomEnd = __romPos + SIZEOF(.name); \
    __romPos += SIZEOF(.name);

/* Starts a noload segment which is intended to contain variables which do not have data yet. */
#define BEGIN_NOLOAD(name) \
    _##name##SegmentBssStart = ADDR(.name.noload); \
    .name.noload (NOLOAD) :

/* Ends a noload segment */
#define END_NOLOAD(name) \
    _##name##SegmentBssEnd = ADDR(.name.noload) + SIZEOF(.name.noload); \
    _##name##SegmentBssSize = SIZEOF(.name.noload);

SECTIONS
{
   __romPos = 0;

   BEGIN_SEG(boot, 0x04000000)
   {
      BUILD_DIR/asm/rom_header.o(.text); /* ROM Header */
      BUILD_DIR/boot.6102.o(.data); /* CIC 6102 bootcode */
   }
   END_SEG(boot)

   BEGIN_SEG(code, NU_SPEC_BOOT_ADDR) SUBALIGN(16)
   {
      /* text */
      BUILD_DIR/asm/entry.o(.start);
      /usr/lib/n64/nusys/nusys_rom.o(.text);
      BUILD_DIR/src*.o(.text);
      target/mips-nintendo64-elf/debug/libminimal.a(.text);

      */libultra_rom.a:*.o(.text);
      */libnustd.a:*.o(.text);
      */libnusys.a:*.o(.text);
#ifdef NAUDIO
      */libnualstl_n.a:*.o(.text);
      */libn_mus.a:*.o(.text);
      */libn_audio_sc.a:*.o(.text);
#else
      */libnualstl.a:*.o(.text);
      */libmus.a:*.o(.text);
#endif
      */libgcc.a:*.o(.text);
      /usr/lib/n64/PR/rspboot.o(.text);
      /usr/lib/n64/PR/aspMain.o(.text);
      /usr/lib/n64/PR/n_aspMain.o(.text);
      /usr/lib/n64/PR/gspF3DEX2.fifo.o(.text);
      /usr/lib/n64/PR/gspL3DEX2.fifo.o(.text);
      /usr/lib/n64/PR/gspF3DEX2.Rej.fifo.o(.text);
      /usr/lib/n64/PR/gspF3DEX2.NoN.fifo.o(.text);
      /usr/lib/n64/PR/gspF3DLX2.Rej.fifo.o(.text);
      /usr/lib/n64/PR/gspS2DEX2.fifo.o(.text);

      /* data */
      /usr/lib/n64/nusys/nusys_rom.o(.data*);
      BUILD_DIR/src*.o(.data*);
      target/mips-nintendo64-elf/debug/libminimal.a(.data*);

      */libultra_rom.a:*.o(.data*);
      */libnustd.a:*.o(.data*);
      */libnusys.a:*.o(.data*);
#ifdef NAUDIO
      */libnualstl_n.a:*.o(.data*);
      */libn_mus.a:*.o(.data*);
      */libn_audio_sc.a:*.o(.data*);
#else
      */libnualstl.a:*.o(.data*);
      */libmus.a:*.o(.data*);
#endif
      /usr/lib/n64/PR/rspboot.o(.data*);
      /usr/lib/n64/PR/aspMain.o(.data*);
      /usr/lib/n64/PR/n_aspMain.o(.data*);
      /usr/lib/n64/PR/gspF3DEX2.fifo.o(.data*);
      /usr/lib/n64/PR/gspL3DEX2.fifo.o(.data*);
      /usr/lib/n64/PR/gspF3DEX2.Rej.fifo.o(.data*);
      /usr/lib/n64/PR/gspF3DEX2.NoN.fifo.o(.data*);
      /usr/lib/n64/PR/gspF3DLX2.Rej.fifo.o(.data*);
      /usr/lib/n64/PR/gspS2DEX2.fifo.o(.data*);

      /* rodata */
      /usr/lib/n64/nusys/nusys_rom.o(.*rodata*);
      BUILD_DIR/src*.o(.*rodata*);
      target/mips-nintendo64-elf/debug/libminimal.a(.rodata*);

      */libultra_rom.a:*.o(.*rodata*);
      */libnustd.a:*.o(*.rodata*);
      */libnusys.a:*.o(.*rodata*);
#ifdef NAUDIO
      */libnualstl_n.a:*.o(.*rodata*);
      */libn_mus.a:*.o(.*rodata*);
      */libn_audio_sc.a:*.o(.*rodata*);
#else
      */libnualstl.a:*.o(.*rodata*);
      */libmus.a:*.o(.*rodata*);
#endif
      */libgcc.a:*.o(.*rodata*);
   }
   END_SEG(code)
   BEGIN_NOLOAD(code)
   {
      /usr/lib/n64/nusys/nusys_rom.o(COMMON);
      /usr/lib/n64/nusys/nusys_rom.o(.scommon);
      /usr/lib/n64/nusys/nusys_rom.o(.*bss*);
      BUILD_DIR/src*.o(.*bss*);
      target/mips-nintendo64-elf/debug/libminimal.a(.bss*);

      */libultra_rom.a:*.o(COMMON);
      */libultra_rom.a:*.o(.scommon);
      */libultra_rom.a:*.o(.*bss*);
      */libnustd.a:*.o(COMMON);
      */libnustd.a:*.o(.scommon);
      */libnustd.a:*.o(.*bss*);
      */libnusys.a:*.o(COMMON);
      */libnusys.a:*.o(.scommon);
      */libnusys.a:*.o(.*bss*);
#ifdef NAUDIO
      */libnualstl_n.a:*.o(COMMON);
      */libnualstl_n.a:*.o(.scommon);
      */libnualstl_n.a:*.o(.*bss*);
      */libn_mus.a:*.o(COMMON);
      */libn_mus.a:*.o(.scommon);
      */libn_mus.a:*.o(.*bss*);
      */libn_audio_sc.a:*.o(COMMON);
      */libn_audio_sc.a:*.o(.scommon);
      */libn_audio_sc.a:*.o(.*bss*);
#else
      */libnualstl.a:*.o(COMMON);
      */libnualstl.a:*.o(.scommon);
      */libnualstl.a:*.o(.*bss*);
      */libmus.a:*.o(COMMON);
      */libmus.a:*.o(.scommon);
      */libmus.a:*.o(.*bss*);
#endif
   }
   END_NOLOAD(code)

}
