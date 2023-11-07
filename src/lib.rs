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
    }, transform::TransformSystem,
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
        .add_systems(PostUpdate, update_transform.before(TransformSystem::TransformPropagate));

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

/// Marks a camera as pixel perfect
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PixelPerfectCamera {
    /// The virtual resolution of the image
    pub resolution: Vec2,
    /// The subpixel translation of the camera, use this instead of [`Transform::translation`]
    pub subpixel_translation: Vec2,
    /// The color of the bars on the edge of the image, supports alpha transparency
    pub bar_color: Color,
    /// The offset in virtual coordinates of the bars, positive values shrink the view, negative expand it
    pub bar_offset: Vec2,
}

impl Default for PixelPerfectCamera {
    fn default() -> Self {
        Self {
            resolution: Vec2::splat(256.),
            subpixel_translation: Default::default(),
            bar_color: Color::BLACK,
            bar_offset: Default::default(),
        }
    }
}

fn update_transform(mut query: Query<(&mut Transform, &mut PixelPerfectCamera)>) {
    for (mut transform, camera) in &mut query {
        transform.translation = camera
            .subpixel_translation
            .floor()
            .extend(transform.translation.z);
    }
}
