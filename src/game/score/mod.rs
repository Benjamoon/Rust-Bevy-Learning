use bevy::prelude::*;

mod systems;
pub mod resources;

use crate::AppState;

use self::systems::*;
use self::resources::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<HighScores>()

            // On Enter
            .add_systems(OnEnter(AppState::Game), insert_score)
            
            // On Exit
            .add_systems(OnExit(AppState::Game), remove_score)

            // On Update
            .add_systems(
                Update,
                (update_score.run_if(in_state(AppState::Game)), update_high_scores, high_scores_updated),
            );
    }
}