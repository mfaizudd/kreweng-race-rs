use std::f32::consts::PI;

use bevy::{color::palettes::css::PURPLE, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, aim_rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(asset_server.load("stone01.png")));
    commands.spawn(AimBundle::new(
        meshes.add(Rectangle::default()),
        materials.add(Color::from(PURPLE)),
    ));
}

fn aim_rotate(time: Res<Time>, mut aims: Query<(&mut Transform, &mut Speed, &mut Aim)>) {
    for (mut transform, mut speed, mut aim) in &mut aims {
        let delta = speed.0 * time.delta_secs();
        aim.0 += delta;
        if aim.0 > PI || aim.0 < 0.0 {
            speed.0 = -speed.0;
        }
        transform.rotate_z(delta);
    }
}

#[derive(Component)]
struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
}

impl PlayerBundle {
    fn new(asset: Handle<Image>) -> Self {
        return Self {
            player: Player,
            sprite: Sprite::from_image(asset),
            transform: Transform::IDENTITY,
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
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

impl AimBundle {
    fn new(mesh: Handle<Mesh>, color: Handle<ColorMaterial>) -> Self {
        return Self {
            aim: Aim(0.0),
            speed: Speed(PI),
            mesh: Mesh2d(mesh),
            material: MeshMaterial2d(color),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
        };
    }
}
