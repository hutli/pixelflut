#include "lvgl/lvgl.h"

/*******************************************************************************
 * Size: 14 px
 * Bpp: 1
 * Opts: 
 ******************************************************************************/

#ifndef INCONSOLATA
#define INCONSOLATA 1
#endif

#if INCONSOLATA

/*-----------------
 *    BITMAPS
 *----------------*/

/*Store the image of the glyphs*/
static LV_ATTRIBUTE_LARGE_CONST const uint8_t gylph_bitmap[] = {
    /* U+41 "A" */
    0x10, 0x20, 0x41, 0x42, 0x85, 0x1f, 0x22, 0x45,
    0x4,

    /* U+42 "B" */
    0xf2, 0x28, 0xbc, 0x8a, 0x18, 0x63, 0xf8,

    /* U+43 "C" */
    0x39, 0x18, 0x20, 0x82, 0x8, 0x11, 0x38,

    /* U+44 "D" */
    0xf2, 0x28, 0x61, 0x86, 0x18, 0x62, 0xf0,

    /* U+45 "E" */
    0xfc, 0x21, 0xf, 0xc2, 0x10, 0xf8,

    /* U+46 "F" */
    0xfc, 0x21, 0xe8, 0x42, 0x10, 0x80,

    /* U+47 "G" */
    0x39, 0x18, 0x20, 0x9e, 0x18, 0x51, 0x38,

    /* U+48 "H" */
    0x8c, 0x63, 0x1f, 0xc6, 0x31, 0x88,

    /* U+49 "I" */
    0xf9, 0x8, 0x42, 0x10, 0x84, 0xf8,

    /* U+4A "J" */
    0x78, 0x84, 0x21, 0x8, 0x42, 0xe0,

    /* U+4B "K" */
    0x8a, 0x4b, 0x28, 0xe2, 0xc9, 0x22, 0x8c,

    /* U+4C "L" */
    0x84, 0x21, 0x8, 0x42, 0x10, 0xf8,

    /* U+4D "M" */
    0x87, 0x3c, 0xed, 0xb6, 0x18, 0x61, 0x84,

    /* U+4E "N" */
    0x87, 0x1e, 0x69, 0x96, 0x58, 0xe3, 0x84,

    /* U+4F "O" */
    0x79, 0x28, 0x61, 0x86, 0x18, 0x52, 0x78,

    /* U+50 "P" */
    0xfa, 0x18, 0x61, 0xfa, 0x8, 0x20, 0x80,

    /* U+51 "Q" */
    0x31, 0x28, 0x61, 0x86, 0x18, 0x52, 0x38, 0x41,
    0xc0,

    /* U+52 "R" */
    0xf4, 0x63, 0x1f, 0x4a, 0x51, 0x88,

    /* U+53 "S" */
    0x79, 0x14, 0x10, 0x38, 0x30, 0x61, 0x78,

    /* U+54 "T" */
    0xfc, 0x82, 0x8, 0x20, 0x82, 0x8, 0x20,

    /* U+55 "U" */
    0x86, 0x18, 0x61, 0x86, 0x18, 0x73, 0x78,

    /* U+56 "V" */
    0x42, 0x89, 0x13, 0x22, 0x85, 0xa, 0x8, 0x10,

    /* U+58 "X" */
    0x8a, 0x65, 0x1c, 0x21, 0x45, 0x22, 0x88,

    /* U+59 "Y" */
    0x45, 0x12, 0x8a, 0x10, 0x41, 0x4, 0x10,

    /* U+5A "Z" */
    0xfc, 0x20, 0x84, 0x20, 0x84, 0x30, 0xfc,

    /* U+C5 "Å" */
    0x30, 0x60, 0xc0, 0x81, 0x6, 0xa, 0x24, 0x78,
    0x8a, 0x14, 0x20,

    /* U+C6 "Æ" */
    0x3c, 0xc5, 0x14, 0x5d, 0x4f, 0x24, 0x9c,

    /* U+D8 "Ø" */
    0x5, 0xe4, 0xa5, 0x96, 0x9a, 0x71, 0x49, 0xe8,
    0x0
};


/*---------------------
 *  GLYPH DESCRIPTION
 *--------------------*/

static const lv_font_fmt_txt_glyph_dsc_t glyph_dsc[] = {
    {.bitmap_index = 0, .adv_w = 0, .box_w = 0, .box_h = 0, .ofs_x = 0, .ofs_y = 0} /* id = 0 reserved */,
    {.bitmap_index = 0, .adv_w = 112, .box_w = 7, .box_h = 10, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 9, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 16, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 23, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 30, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 36, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 42, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 49, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 55, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 61, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 67, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 74, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 80, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 87, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 94, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 101, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 108, .adv_w = 112, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = -2},
    {.bitmap_index = 117, .adv_w = 112, .box_w = 5, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 123, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 130, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 137, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 144, .adv_w = 112, .box_w = 7, .box_h = 9, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 152, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 159, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 166, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 173, .adv_w = 112, .box_w = 7, .box_h = 12, .ofs_x = 0, .ofs_y = 0},
    {.bitmap_index = 184, .adv_w = 112, .box_w = 6, .box_h = 9, .ofs_x = 1, .ofs_y = 0},
    {.bitmap_index = 191, .adv_w = 112, .box_w = 6, .box_h = 11, .ofs_x = 1, .ofs_y = -1}
};

/*---------------------
 *  CHARACTER MAPPING
 *--------------------*/

static const uint16_t unicode_list_1[] = {
    0x0, 0x1, 0x2, 0x6d, 0x6e, 0x80
};

/*Collect the unicode lists and glyph_id offsets*/
static const lv_font_fmt_txt_cmap_t cmaps[] =
{
    {
        .range_start = 65, .range_length = 22, .glyph_id_start = 1,
        .unicode_list = NULL, .glyph_id_ofs_list = NULL, .list_length = 0, .type = LV_FONT_FMT_TXT_CMAP_FORMAT0_TINY
    },
    {
        .range_start = 88, .range_length = 129, .glyph_id_start = 23,
        .unicode_list = unicode_list_1, .glyph_id_ofs_list = NULL, .list_length = 6, .type = LV_FONT_FMT_TXT_CMAP_SPARSE_TINY
    }
};



/*--------------------
 *  ALL CUSTOM DATA
 *--------------------*/

/*Store all the custom data of the font*/
static lv_font_fmt_txt_dsc_t font_dsc = {
    .glyph_bitmap = gylph_bitmap,
    .glyph_dsc = glyph_dsc,
    .cmaps = cmaps,
    .kern_dsc = NULL,
    .kern_scale = 0,
    .cmap_num = 2,
    .bpp = 1,
    .kern_classes = 0,
    .bitmap_format = 0
};


/*-----------------
 *  PUBLIC FONT
 *----------------*/

/*Initialize a public general font descriptor*/
lv_font_t inconsolata = {
    .get_glyph_dsc = lv_font_get_glyph_dsc_fmt_txt,    /*Function pointer to get glyph's data*/
    .get_glyph_bitmap = lv_font_get_bitmap_fmt_txt,    /*Function pointer to get glyph's bitmap*/
    .line_height = 14,          /*The maximum line height required by the font*/
    .base_line = 2,             /*Baseline measured from the bottom of the line*/
#if !(LVGL_VERSION_MAJOR == 6 && LVGL_VERSION_MINOR == 0)
    .subpx = LV_FONT_SUBPX_NONE,
#endif
    .dsc = &font_dsc           /*The custom font data. Will be accessed by `get_glyph_bitmap/dsc` */
};

#endif /*#if INCONSOLATA*/

