use bevy::app::{ Plugin, Startup, Update };
use bevy::asset::AssetServer;
use bevy::audio::{ AudioBundle, AudioSource };
use bevy::ecs::component::Component;
use bevy::ecs::query::With;
use bevy::ecs::system::{ Commands, Query, Res };
use bevy::math::{ vec2, vec3, Vec2 };

use bevy::sprite::SpriteBundle;
use bevy::time::Time;
use bevy::transform::components::Transform;
use bevy::window::{ PrimaryWindow, Window };
use rand::random;

use crate::{ ENEMY_SPEED, HALF_ENEMY_SIZE, NUMBER_OF_ENEMIES };

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement)
            .add_systems(Update, update_enemy_direction)
            .add_systems(Update, confine_enemy_movement);
    }
}

pub fn spawn_enemies(
    mut commands: Commands<'_, '_>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: vec2(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = vec3(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    let window = window_query.get_single().unwrap();

    let x_min = 0.0 + HALF_ENEMY_SIZE;
    let x_max = window.width() - HALF_ENEMY_SIZE;
    let y_min = 0.0 + HALF_ENEMY_SIZE;
    let y_max = window.height() - HALF_ENEMY_SIZE;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sound_effect_1 = asset_server.load::<AudioSource>("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load::<AudioSource>("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 { sound_effect_1 } else { sound_effect_2 };
            commands.spawn(AudioBundle {
                source: sound_effect,
                ..Default::default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    for mut transform in enemy_query.iter_mut() {
        if transform.translation.x < HALF_ENEMY_SIZE {
            transform.translation.x = HALF_ENEMY_SIZE;
        }
        if transform.translation.x > window.width() - HALF_ENEMY_SIZE {
            transform.translation.x = window.width() - HALF_ENEMY_SIZE;
        }
        if transform.translation.y < HALF_ENEMY_SIZE {
            transform.translation.y = HALF_ENEMY_SIZE;
        }
        if transform.translation.y > window.height() - HALF_ENEMY_SIZE {
            transform.translation.y = window.height() - HALF_ENEMY_SIZE;
        }
    }
}
