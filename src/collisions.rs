use bevy::{
    app::{ Plugin, Update },
    asset::AssetServer,
    audio::{ AudioBundle, AudioSource },
    ecs::{ entity::Entity, query::With, system::{ Commands, Query, Res, ResMut } },
    transform::components::Transform,
};

use crate::{
    enemy::Enemy,
    player::Player,
    score::Score,
    star::Star,
    HALF_ENEMY_SIZE,
    HALF_PLAYER_SIZE,
    HALF_STAR_SIZE,
};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, enemy_hit_player).add_systems(Update, player_collect_star);
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            if distance < HALF_ENEMY_SIZE + HALF_PLAYER_SIZE {
                println!("Enemy hit player! Game Over!");
                let sound_effect = asset_server.load::<AudioSource>(
                    "audio/explosionCrunch_000.ogg"
                );
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..Default::default()
                });
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_collect_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in enemy_query.iter() {
            let distance = player_transform.translation.distance(star_transform.translation);
            if distance < HALF_STAR_SIZE + HALF_PLAYER_SIZE {
                println!("You collected a star!");
                let sound_effect = asset_server.load::<AudioSource>("audio/toggle_001.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..Default::default()
                });
                score.value += 1;
                commands.entity(star_entity).despawn();
            }
        }
    }
}
