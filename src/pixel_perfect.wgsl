#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::view::View;

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
struct PixelPerfectCamera {
    resolution: vec2<f32>,
    subpixel_position: vec2<f32>,
    bar_color: vec4<f32>,
}
@group(0) @binding(2) var<uniform> camera: PixelPerfectCamera;
@group(0) @binding(3) var<uniform> view: View;

fn max_component(v: vec2<f32>) -> f32 {
    return max(v.x, v.y);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let scaling_vec = view.viewport.zw / camera.resolution;
    let scaling = min(scaling_vec.x, scaling_vec.y);

    // (-w / 2, -h / 2) -> (w / 2, h / 2)
    let screen_coordinates = view.viewport.xy + (in.uv - 0.5) * view.viewport.zw;

    // (-1, -1) -> (1, 1) in scaled coords
    let scaled_virtual = screen_coordinates / (scaling * 0.5 * camera.resolution);

    let bar_mask = step(1.0, max_component(abs(scaled_virtual)));

    let color = textureSample(screen_texture, texture_sampler, (in.uv - 0.5) / scaling + 0.5);

    return mix(color, camera.bar_color, bar_mask);
}