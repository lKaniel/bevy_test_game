mod player;
mod enemy;
mod star;
mod collisions;
mod score;

use bevy::app::App;
use bevy::DefaultPlugins;
use collisions::CollisionsPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::{ Score, ScoresPlugin };
use star::StarPlugin;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const HALF_PLAYER_SIZE: f32 = PLAYER_SIZE / 2.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const HALF_ENEMY_SIZE: f32 = ENEMY_SIZE / 2.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const HALF_STAR_SIZE: f32 = STAR_SIZE / 2.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub fn run_app() {
    App::new()
        .init_resource::<Score>()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(StarPlugin)
        .add_plugins(CollisionsPlugin)
        .add_plugins(ScoresPlugin)
        .run();
}
