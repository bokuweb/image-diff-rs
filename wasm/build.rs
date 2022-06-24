extern crate cc;

fn main() {
    cc::Build::new()
        .file("../libwebp/src/enc/tree_enc.c")
        .file("../libwebp/src/enc/token_enc.c")
        .file("../libwebp/src/enc/analysis_enc.c")
        .file("../libwebp/src/enc/webp_enc.c")
        .file("../libwebp/src/enc/picture_enc.c")
        .file("../libwebp/src/enc/quant_enc.c")
        .file("../libwebp/src/enc/alpha_enc.c")
        .file("../libwebp/src/enc/frame_enc.c")
        .file("../libwebp/src/enc/syntax_enc.c")
        .file("../libwebp/src/enc/cost_enc.c")
        .file("../libwebp/src/enc/filter_enc.c")
        .file("../libwebp/src/enc/iterator_enc.c")
        .file("../libwebp/src/enc/vp8l_enc.c")
        .file("../libwebp/src/enc/near_lossless_enc.c")
        .file("../libwebp/src/enc/histogram_enc.c")
        .file("../libwebp/src/enc/backward_references_enc.c")
        .file("../libwebp/src/enc/backward_references_cost_enc.c")
        .file("../libwebp/src/enc/predictor_enc.c")
        .file("../libwebp/src/enc/picture_tools_enc.c")
        .file("../libwebp/src/enc/config_enc.c")
        .file("../libwebp/src/enc/picture_csp_enc.c")
        .file("../libwebp/src/utils/filters_utils.c")
        .file("../libwebp/src/utils/huffman_encode_utils.c")
        .file("../libwebp/src/dec/alpha_dec.c")
        .file("../libwebp/src/dsp/lossless_enc.c")
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
        .file("../libwebp/src/dsp/enc_sse2.c")
        .file("../libwebp/src/dsp/enc.c")
        .file("../libwebp/src/dsp/ssim.c")
        .file("../libwebp/src/utils/utils.c")
        .file("../libwebp/src/utils/rescaler_utils.c")
        .file("../libwebp/src/utils/bit_reader_utils.c")
        .file("../libwebp/src/utils/thread_utils.c")
        .file("../libwebp/src/utils/huffman_utils.c")
        .file("../libwebp/src/utils/color_cache_utils.c")
        .file("../libwebp/src/utils/random_utils.c")
        .file("../libwebp/src/utils/quant_levels_dec_utils.c")
        .file("../libwebp/src/utils/quant_levels_utils.c")
        .file("../libwebp/src/utils/bit_writer_utils.c")
        .file("../libwebp/src/dsp/cost.c")
        .include("../libwebp")
        .compile("test");
}
