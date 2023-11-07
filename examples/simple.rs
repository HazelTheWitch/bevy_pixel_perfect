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

#[derive(Component)]
struct Rotate;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PixelPerfectCameraBundle {
        pixel_camera: PixelPerfectCamera {
            resolution: Vec2::splat(64.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("bevy_pixel.png"),
            transform: Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
            ..Default::default()
        },
        Rotate,
    ));
}

fn modify_resolution(
    mut query: Query<&mut PixelPerfectCamera>,
    time: Res<Time>,
) {
    for mut camera in &mut query {
        camera.resolution = Vec2::splat(16.0 * time.elapsed_seconds().sin() + 64.0);
        camera.subpixel_translation.x = 16.0 * (time.elapsed_seconds() / 2.0).sin()
    }
}
