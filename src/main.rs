use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_rotate)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(PlayerBundle::new(asset_server.load("stone01.png")));
}

fn player_rotate(
    mut query: Single<&mut Transform, With<Player>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
    if let Some(position) = window.cursor_position() {
        let mut pos = center - position;
        pos.x = -pos.x;
        query.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, pos.to_angle() - PI / 2.0)
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
