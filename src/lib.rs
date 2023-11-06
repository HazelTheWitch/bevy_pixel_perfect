mod render;

use bevy::{prelude::*, render::{extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin}, render_resource::ShaderType, RenderApp, render_graph::{RenderGraphApp, ViewNodeRunner}}, asset::load_internal_asset, core_pipeline::core_2d};
use render::PixelPerfectPipeline;

use crate::render::{PIXEL_PERFECT_SHADER_HANDLE, PixelPerfectNode};

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
        ));

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

#[derive(Bundle, Default)]
pub struct PixelPerfectCameraBundle {
    pub pixel_camera: PixelPerfectCamera,
    pub camera: Camera2dBundle,
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct PixelPerfectCamera {
    pub resolution: Vec2,
    pub subpixel_position: Vec2,
    pub bar_color: Color,
}
