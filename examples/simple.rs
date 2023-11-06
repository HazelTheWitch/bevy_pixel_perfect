use bevy::prelude::*;
use bevy_pixel_perfect::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Add plugin to add the post processing effect
        .add_plugins(PixelPerfectPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (translate_sprite, translate_camera))
        .run();
}

#[derive(Component)]
struct Rotate;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn translate_sprite(mut query: Query<&mut Transform, With<Rotate>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.translation.y = (2.0 * time.elapsed_seconds()).sin() * 16.0;
    }
}

fn translate_camera(mut cameras: Query<&mut PixelPerfectCamera>, time: Res<Time>) {
    for mut camera in &mut cameras {
        camera.subpixel_position.x = (time.elapsed_seconds() / 2.0).sin() * 64.0;
    }
}
