extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("../core/src/test.cpp")
        .file("../libwebp/src/enc/webp_enc.c")
        .file("../libwebp/src/dec/alpha_dec.c")
        .file("../libwebp/src/dec/webp_dec.c")
        .file("../libwebp/src/dec/buffer_dec.c")
        .file("../libwebp/src/dec/io_dec.c")
        .file("../libwebp/src/dec/vp8_dec.c")
        .file("../libwebp/src/dec/vp8l_dec.c")
        .file("../libwebp/src/dec/frame_dec.c")
        .file("../libwebp/src/dec/quant_dec.c")
        .file("../libwebp/src/dec/tree_dec.c")
        .file("../libwebp/src/dsp/rescaler.c")
        .file("../libwebp/src/dsp/rescaler_sse2.c")
        .file("../libwebp/src/dsp/dec.c")
        .file("../libwebp/src/dsp/dec_sse2.c")
        .file("../libwebp/src/dsp/dec_clip_tables.c")
        .file("../libwebp/src/dsp/yuv_mips_dsp_r2.c")
        .file("../libwebp/src/dsp/yuv.c")
        .file("../libwebp/src/dsp/yuv_sse2.c")
        .file("../libwebp/src/dsp/alpha_processing.c")
        .file("../libwebp/src/dsp/alpha_processing_sse2.c")
        .file("../libwebp/src/dsp/upsampling_msa.c")
        .file("../libwebp/src/dsp/upsampling_neon.c")
        .file("../libwebp/src/dsp/upsampling.c")
        .file("../libwebp/src/dsp/upsampling_sse2.c")
        .file("../libwebp/src/dsp/cpu.c")
        .file("../libwebp/src/dsp/filters.c")
        .file("../libwebp/src/dsp/filters_sse2.c")
        .file("../libwebp/src/dsp/lossless.c")
        .file("../libwebp/src/dsp/lossless_sse2.c")
        .file("../libwebp/src/utils/utils.c")
        .file("../libwebp/src/utils/rescaler_utils.c")
        .file("../libwebp/src/utils/bit_reader_utils.c")
        .file("../libwebp/src/utils/thread_utils.c")
        .file("../libwebp/src/utils/huffman_utils.c")
        .file("../libwebp/src/utils/color_cache_utils.c")
        .file("../libwebp/src/utils/random_utils.c")
        .file("../libwebp/src/utils/quant_levels_dec_utils.c")
        .include("../libwebp")
        .compile("test");
}
