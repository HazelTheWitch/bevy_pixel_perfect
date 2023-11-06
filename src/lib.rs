mod render;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d,
    prelude::*,
    render::{
        extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        render_resource::ShaderType,
        RenderApp,
    },
};
use render::PixelPerfectPipeline;

use crate::render::{PixelPerfectNode, PIXEL_PERFECT_SHADER_HANDLE};

/// A pixel perfect post processing effect based on [Aarthificial's Astortion renderer](https://www.youtube.com/watch?v=jguyR4yJb1M).
///
/// Be sure to set image sampling mode to nearest when using this plugin!
pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        load_internal_asset!(
            app,
            PIXEL_PERFECT_SHADER_HANDLE,
            "pixel_perfect.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((
            ExtractComponentPlugin::<PixelPerfectCamera>::default(),
            UniformComponentPlugin::<PixelPerfectCamera>::default(),
        ))
        .insert_resource(Msaa::Off)
        .add_systems(Update, update_transform);

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PixelPerfectNode>>(
                core_2d::graph::NAME,
                PixelPerfectNode::NAME,
            )
            .add_render_graph_edges(
                core_2d::graph::NAME,
                &[
                    core_2d::graph::node::TONEMAPPING,
                    PixelPerfectNode::NAME,
                    core_2d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PixelPerfectPipeline>();
    }
}

/// Minimal pixel perfect camera bundle
#[derive(Bundle, Default)]
pub struct PixelPerfectCameraBundle {
    pub pixel_camera: PixelPerfectCamera,
    pub camera: Camera2dBundle,
}

/// Set desired virtual resolution with `resolution` and move it with `subpixel_position`. Set the bar color with `bar_color`.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PixelPerfectCamera {
    pub resolution: Vec2,
    pub subpixel_position: Vec2,
    pub bar_color: Color,
}

impl Default for PixelPerfectCamera {
    fn default() -> Self {
        Self {
            resolution: Vec2::splat(256.),
            subpixel_position: Default::default(),
            bar_color: Color::BLACK,
        }
    }
}

fn update_transform(mut query: Query<(&mut Transform, &mut PixelPerfectCamera)>) {
    for (mut transform, camera) in &mut query {
        transform.translation = camera
            .subpixel_position
            .floor()
            .extend(transform.translation.z);
    }
}
