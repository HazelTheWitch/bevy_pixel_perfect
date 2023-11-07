//! This example simply shows that ui functions properly over the [`PixelPerfectPlugin`](`bevy_pixel_perfect::PixelPerfectPlugin`)

use bevy::prelude::*;
use bevy_pixel_perfect::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPerfectPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PixelPerfectCameraBundle {
        pixel_camera: PixelPerfectCamera {
            resolution: Vec2::splat(128.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("bevy_pixel.png"),
            ..Default::default()
        }
    );

    commands.spawn(
        TextBundle::from_section(
            "Hello World!",
            TextStyle {
                font_size: 64.0,
                ..Default::default()
            }).with_style(Style { 
                position_type: PositionType::Absolute,
                bottom: Val::Px(5.0),
                right: Val::Px(5.0),
                ..Default::default()
        }),
    );
}