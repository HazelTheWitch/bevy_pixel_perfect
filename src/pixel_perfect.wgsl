#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PixelPerfectCamera {
    resolution: vec2<f32>,
    subpixel_position: vec2<f32>,
    bar_color: vec4<f32>,
    bar_offset: vec2<f32>,
}
@group(0) @binding(2) var<uniform> camera: PixelPerfectCamera;
@group(0) @binding(3) var<uniform> view: View;

fn max_component(v: vec2<f32>) -> f32 {
    return max(v.x, v.y);
}

fn min_component(v: vec2<f32>) -> f32 {
    return min(v.x, v.y);
}

fn sample_bilinear_grad(screen_texture: texture_2d<f32>, texture_sampler: sampler, uv: vec2<f32>, ddx: vec2<f32>, ddy: vec2<f32>, texel_size: vec4<f32>) -> vec4<f32> {
    let uv_texels = uv * texel_size.zw + 0.5;

    let uv_min_max = vec4(uv - texel_size.xy * 0.49, uv + texel_size.xy * 0.49);

    let blend_factor = fract(uv_texels);

    let texel_a = textureSampleGrad(screen_texture, texture_sampler, uv_min_max.xy, ddx, ddy);
    let texel_b = textureSampleGrad(screen_texture, texture_sampler, uv_min_max.xw, ddx, ddy);
    let texel_c = textureSampleGrad(screen_texture, texture_sampler, uv_min_max.zy, ddx, ddy);
    let texel_d = textureSampleGrad(screen_texture, texture_sampler, uv_min_max.zw, ddx, ddy);

    return mix(mix(texel_a, texel_b, blend_factor.y), mix(texel_c, texel_d, blend_factor.y), blend_factor.x);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let viewport = vec4(vec2(1.0) / view.viewport.zw, view.viewport.zw);

    let scaling_vec = viewport.zw / camera.resolution;
    let scaling = min_component(scaling_vec);

    // (-w / 2, -h / 2) -> (w / 2, h / 2)
    let screen_coordinates = (in.uv - 0.5) * viewport.zw;
    
    // 1 if in the bars, 0 otherwise
    let bar_mask = max_component(step(camera.resolution, 2.0 / scaling * abs(screen_coordinates) + camera.bar_offset));

    let scaled_uv = (in.uv - 0.5) / scaling + fract(camera.subpixel_position) * viewport.xy + 0.5;

#ifdef BILINEAR
    let box_size = clamp(fwidth(scaled_uv) * viewport.zw, vec2(1.0e-5), vec2(1.0));
    
    let tx = scaled_uv * viewport.zw - 0.5 * box_size;
    let tx_offset = smoothstep(vec2(1.0) - box_size, vec2(1.0), fract(tx));
    
    let adjusted_uv = (floor(tx) + 0.5 + tx_offset) * viewport.xy;

    let color = sample_bilinear_grad(screen_texture, texture_sampler, adjusted_uv, dpdx(in.uv), dpdy(in.uv), viewport);
#else
    let color = textureSample(screen_texture, texture_sampler, scaled_uv);
#endif

    // The color with bars applied but before the "hard" bars at end of the screen
    return mix(color, camera.bar_color, bar_mask * camera.bar_color.a);
}