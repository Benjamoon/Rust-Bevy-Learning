use bevy::prelude::*;

use crate::AppState;

use self::resources::*;
use self::systems::*;

use super::SimulationState;

pub mod components;
mod resources;
mod systems;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 350.0;
pub const ENEMY_SIZE: f32 = 64.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // On enter game state, create enemies
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            // On leave game state, despawn enemies
            .add_systems(OnExit(AppState::Game), despawn_enemies)

            // Update systems (Only when game state is game and running)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                )
                    .chain()
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (tick_enemy_spawn_timer, spawn_enemies_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .init_resource::<EnemySpawnTimer>();
    }
}
