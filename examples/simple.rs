use bevy::prelude::*;
use bevy_pixel_perfect::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add plugin to add the post processing effect
        .add_plugins(PixelPerfectPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct Rotate;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(PixelPerfectCameraBundle {
        pixel_camera: PixelPerfectCamera { resolution: Vec2::splat(256.), subpixel_position: Vec2::ZERO, bar_color: Color::BLACK },
        ..Default::default()
    });

    commands.spawn((SpriteBundle {
        texture: asset_server.load("bevy.png"),
        ..Default::default()
    }, Rotate));
}

fn rotate(
    mut query: Query<&mut Transform, With<Rotate>>,
    time: Res<Time>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_axis_angle(Vec3::Z, std::f32::consts::TAU * time.elapsed_seconds().sin());
    }
}
