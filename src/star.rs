use bevy::{
    app::{ Plugin, Startup },
    asset::AssetServer,
    ecs::{ component::Component, query::With, system::{ Commands, Query, Res, Resource } },
    sprite::SpriteBundle,
    time::{ Timer, TimerMode },
    transform::components::Transform,
    window::{ PrimaryWindow, Window },
};
use rand::random;

use crate::{ NUMBER_OF_STARS, STAR_SPAWN_TIME };

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<StarSpawnTimer>().add_systems(Startup, spawn_stars);
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Star {}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}
