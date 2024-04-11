use bevy::prelude::*;

pub mod components;
mod systems;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter
            .add_systems(OnEnter(AppState::Game), spawn_player)
            
            // On Exit
            .add_systems(OnExit(AppState::Game), despawn_player)

            // On update
            .add_systems(
            Update,
            (
                (player_movement, confine_player_movement).chain(),
                enemy_hit_player,
                player_hit_star,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}
