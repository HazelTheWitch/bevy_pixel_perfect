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

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let scaling_vec = view.viewport.zw / camera.resolution;
    let scaling = min_component(scaling_vec);

    // (-w / 2, -h / 2) -> (w / 2, h / 2)
    let screen_coordinates = (in.uv - 0.5) * view.viewport.zw;
    
    // 1 if in the bars, 0 otherwise
    let bar_mask = max_component(step(camera.resolution, 2.0 / scaling * abs(screen_coordinates) + camera.bar_offset));

    let scaled_uv = (in.uv - 0.5) / scaling + fract(camera.subpixel_position) / view.viewport.zw + 0.5;

    let color = textureSample(screen_texture, texture_sampler, scaled_uv);

    // The color with bars applied but before the "hard" bars at end of the screen
    return mix(color, camera.bar_color, bar_mask * camera.bar_color.a);
}