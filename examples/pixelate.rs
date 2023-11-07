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
}

fn modify_resolution(mut query: Query<(&mut PixelPerfectCamera, &mut OrthographicProjection)>, time: Res<Time>) {
    for (mut camera, mut projection) in &mut query {
        let size = 63.0 * time.elapsed_seconds().sin() + 64.0;

        camera.resolution = Vec2::splat(size);
        projection.scale = 128.0 / size;
    }
}