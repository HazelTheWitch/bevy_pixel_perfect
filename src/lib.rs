mod render;

use bevy::{
    asset::load_internal_asset,
    core_pipeline::core_2d,
    prelude::*,
    render::{
        camera::CameraUpdateSystem,
        extract_component::{ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        render_resource::ShaderType,
        RenderApp,
    },
    transform::TransformSystem,
};
use render::PixelPerfectPipeline;

use crate::render::{PixelPerfectNode, PIXEL_PERFECT_SHADER_HANDLE};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PixelPerfectSet {
    Pixelation,
    TransformPropagate,
}

/// A pixel perfect post processing effect based on [Aarthificial's Astortion renderer](https://www.youtube.com/watch?v=jguyR4yJb1M).
///
/// Be sure to set image sampling mode to nearest when using this plugin!
pub struct PixelPerfectPlugin;

impl Plugin for PixelPerfectPlugin {
    fn build(
        &self,
        app: &mut bevy::prelude::App,
    ) {
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
        .configure_sets(
            PostUpdate,
            (
                PixelPerfectSet::Pixelation,
                PixelPerfectSet::TransformPropagate,
            )
                .chain(),
        )
        .configure_sets(
            PostUpdate,
            PixelPerfectSet::Pixelation.before(CameraUpdateSystem),
        )
        .configure_sets(
            PostUpdate,
            PixelPerfectSet::TransformPropagate.before(TransformSystem::TransformPropagate),
        )
        .add_systems(
            PostUpdate,
            (pixelate_added, update_pixelation_resolution)
                .chain()
                .in_set(PixelPerfectSet::Pixelation),
        )
        .add_systems(
            PostUpdate,
            update_transform.in_set(PixelPerfectSet::TransformPropagate),
        );

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

    fn finish(
        &self,
        app: &mut App,
    ) {
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

/// Component which causes a pixelation effect of the virtual resolution, add it to the camera along with [`PixelPerfectCamera`].
#[derive(Component, Default)]
pub struct PixelPerfectPixelation {
    /// Represents the number of times pixels are joined together in the pixelation effect. Must be non-negative.
    ///
    /// resolution = starting_resolution / 2 ^ joins
    pub joins: f32,
    starting_resolution: Vec2,
}

impl PixelPerfectPixelation {
    pub fn from_joins(joins: f32) -> Self {
        Self {
            joins,
            ..Default::default()
        }
    }
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

fn update_transform(
    mut query: Query<(&mut Transform, &PixelPerfectCamera), Changed<PixelPerfectCamera>>
) {
    for (mut transform, camera) in &mut query {
        transform.translation = camera
            .subpixel_translation
            .floor()
            .extend(transform.translation.z);
    }
}

fn pixelate_added(
    mut query: Query<
        (&mut PixelPerfectPixelation, &PixelPerfectCamera),
        Added<PixelPerfectPixelation>,
    >
) {
    for (mut pixelation, camera) in &mut query {
        pixelation.starting_resolution = camera.resolution;
        println!("Set Starting resolution {}", camera.resolution);
    }
}

fn update_pixelation_resolution(
    mut query: Query<
        (
            &mut PixelPerfectCamera,
            &mut OrthographicProjection,
            &PixelPerfectPixelation,
        ),
        Changed<PixelPerfectPixelation>,
    >
) {
    for (mut camera, mut projection, pixelation) in &mut query {
        assert!(pixelation.joins >= 0.0, "joins must be non-negative");

        let scale_factor = 2.0f32.powf(-pixelation.joins);

        camera.resolution = pixelation.starting_resolution * scale_factor;
        projection.scale = 1.0 / scale_factor;
    }
}
