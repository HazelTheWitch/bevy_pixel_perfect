use std::time::Duration;

use bevy::prelude::*;
use bevy_pixel_perfect::*;
use bevy_tweening::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(PixelPerfectPlugin)
        .add_plugins(TweeningPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, component_animator_system::<PixelPerfectCamera>.in_set(AnimationSystem::AnimationUpdate))
        .run();
}

struct BarsLens {
    start_offset: Vec2,
    end_offset: Vec2,
    pixelated: bool,
}

impl Lens<PixelPerfectCamera> for BarsLens {
    fn lerp(&mut self, target: &mut PixelPerfectCamera, ratio: f32) {
        target.bar_offset = self.start_offset + (self.end_offset - self.start_offset) * ratio;

        if self.pixelated {
            target.bar_offset = target.bar_offset.round();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let open_little = Tween::new(
        EaseFunction::QuadraticIn,
        Duration::from_secs(2),
        BarsLens { start_offset: Vec2::splat(128.0), end_offset: Vec2::new(48.0, 112.0), pixelated: true },
    );

    let open_full = Tween::new(
        EaseFunction::QuadraticOut,
        Duration::from_secs(1),
        BarsLens { start_offset: Vec2::new(48.0, 112.0), end_offset: Vec2::ZERO, pixelated: true },
    );
    
    commands.spawn((PixelPerfectCameraBundle {
        pixel_camera: PixelPerfectCamera {
            resolution: Vec2::splat(128.),
            ..Default::default()
        },
        ..Default::default()
    },
    Animator::new(open_little.then(open_full))));

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
