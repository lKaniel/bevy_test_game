use bevy::{
    app::{ Plugin, Update },
    ecs::{ change_detection::DetectChanges, system::{ Res, Resource } },
};

pub struct ScoresPlugin;

impl Plugin for ScoresPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, update_score);
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            value: 0,
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}
