use bevy::prelude::*;
use bevy_pixel_perfect::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPerfectPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, modify_camera)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PixelPerfectCameraBundle {
        pixel_camera: PixelPerfectCamera {
            resolution: Vec2::splat(128.),
            bar_color: Color::BLACK.with_a(0.5),
            ..Default::default()
        },
        ..Default::default()
    });

    for i in -2..=2 {
        commands.spawn(
            SpriteBundle {
                texture: asset_server.load("bevy_pixel.png"),
                transform: Transform::from_translation(Vec3::new(i as f32 * 128.0, 0.0, 0.0)),
                ..Default::default()
            },
        );
    }
}

fn modify_camera(mut cameras: Query<&mut PixelPerfectCamera>, time: Res<Time>) {
    for mut camera in &mut cameras {
        camera.subpixel_position.x = (time.elapsed_seconds() / 2.0).sin() * 64.0;
        camera.bar_offset.x = (time.elapsed_seconds() / 4.0).sin() * 48.0;
    }
}
