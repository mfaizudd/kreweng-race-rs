use std::f32::consts::{PI, TAU};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, aim_rotate)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(asset_server.load("stone01.png")));
    commands.spawn(AimBundle::new()).with_children(|s| {
        s.spawn(
            SpriteBundle::new(asset_server.load("placeholder.png"))
                .with_scale(Vec3::new(1.0, 0.5, 1.0))
                .with_pos(Vec3::new(32.0, 0.0, 0.0)),
        )
        .insert(Collider::cuboid(32.0, 32.0));
    });
}

fn aim_rotate(time: Res<Time>, mut aims: Query<(&mut Transform, &mut Speed, &mut Aim)>) {
    for (mut transform, mut speed, mut aim) in &mut aims {
        let delta = speed.0 * time.delta_secs();
        aim.0 += delta;
        if aim.0 > PI || aim.0 < 0.0 {
            aim.0 = aim.0.clamp(0.0, PI);
            speed.0 = -speed.0;
        }
        transform.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, aim.0);
    }
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
}

impl PlayerBundle {
    fn new(image: Handle<Image>) -> Self {
        return Self {
            player: Player,
            sprite: Sprite::from_image(image),
            transform: Transform::default(),
            collider: Collider::ball(28.0),
        };
    }
}

#[derive(Component)]
struct Aim(f32);

#[derive(Component)]
struct Speed(f32);

#[derive(Bundle)]
struct AimBundle {
    aim: Aim,
    speed: Speed,
    transform: Transform,
}

impl AimBundle {
    fn new() -> Self {
        return Self {
            aim: Aim(0.0),
            speed: Speed(TAU),
            transform: Transform::default(),
        };
    }
}

#[derive(Bundle)]
struct SpriteBundle {
    sprite: Sprite,
    transform: Transform,
}

impl SpriteBundle {
    fn new(image: Handle<Image>) -> Self {
        return Self {
            sprite: Sprite::from_image(image),
            transform: Transform::default(),
        };
    }

    #[inline]
    #[must_use]
    fn with_pos(mut self, pos: Vec3) -> Self {
        self.transform.translation = pos;
        return self;
    }

    #[inline]
    #[must_use]
    fn with_scale(mut self, scale: Vec3) -> Self {
        self.transform.scale = scale;
        return self;
    }
}
