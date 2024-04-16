use bevy::app::{ Plugin, Startup, Update };
use bevy::asset::AssetServer;
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{ Commands, Query, Res };
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::math::Vec3;

use bevy::sprite::SpriteBundle;
use bevy::time::Time;
use bevy::transform::components::Transform;
use bevy::window::{ PrimaryWindow, Window };

use crate::{ HALF_PLAYER_SIZE, PLAYER_SPEED };

#[derive(Component)]
pub struct Player {}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, player_movement)
            .add_systems(Update, confine_player_movement);
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        if transform.translation.x < HALF_PLAYER_SIZE {
            transform.translation.x = HALF_PLAYER_SIZE;
        }
        if transform.translation.x > window.width() - HALF_PLAYER_SIZE {
            transform.translation.x = window.width() - HALF_PLAYER_SIZE;
        }
        if transform.translation.y < HALF_PLAYER_SIZE {
            transform.translation.y = HALF_PLAYER_SIZE;
        }
        if transform.translation.y > window.height() - HALF_PLAYER_SIZE {
            transform.translation.y = window.height() - HALF_PLAYER_SIZE;
        }
    }
}
