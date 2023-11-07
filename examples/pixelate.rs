use bevy::prelude::*;
use bevy_pixel_perfect::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPerfectPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, modify_resolution)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        PixelPerfectCameraBundle {
            pixel_camera: PixelPerfectCamera {
                resolution: Vec2::splat(256.),
                ..Default::default()
            },
            ..Default::default()
        },
        PixelPerfectPixelation::default(),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("bevy.png"),
        ..Default::default()
    });
}

fn modify_resolution(
    mut query: Query<&mut PixelPerfectPixelation>,
    keys: Res<Input<KeyCode>>,
) {
    let amount = if keys.pressed(KeyCode::ShiftLeft) {
        0.2
    } else {
        1.0
    };

    if keys.just_pressed(KeyCode::W) {
        let mut pixelation = query.single_mut();
        pixelation.joins = (pixelation.joins + amount).min(6.0);
    } else if keys.just_pressed(KeyCode::S) {
        let mut pixelation = query.single_mut();
        pixelation.joins = (pixelation.joins - amount).max(0.0);
    }
}
