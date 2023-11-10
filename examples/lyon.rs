use bevy::prelude::*;
use bevy_pixel_perfect::*;
use bevy_prototype_lyon::prelude::*;

const LINES: usize = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPerfectPlugin)
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate)
        .run();
}

#[derive(Component)]
struct Rotate;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(PixelPerfectCameraBundle {
        pixel_camera: PixelPerfectCamera {
            resolution: Vec2::splat(64.),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn((Rotate, SpatialBundle::default()))
        .with_children(|p| {
            for i in 0..LINES {
                let percent = i as f32 / LINES as f32;

                let mut builder = PathBuilder::new();

                builder.move_to(Vec2::ZERO);
                builder.line_to(Vec2::new(24.0, 0.0));

                p.spawn((
                    ShapeBundle {
                        path: builder.build(),
                        spatial: SpatialBundle::from_transform(Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::TAU * percent))),
                        ..Default::default()
                    },
                    Stroke::new(Color::hsl(percent * 360., 1., 0.5), 1.),
                ));
            }
        });
}

fn rotate(
    mut query: Query<&mut Transform, With<Rotate>>,
    time: Res<Time>,
) {
    let rotation = Quat::from_rotation_z(time.elapsed_seconds());

    for mut transform in &mut query {
        transform.rotation = rotation;
    }
}
