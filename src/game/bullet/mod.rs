use bevy::prelude::*;

mod components;
mod systems;

use crate::AppState;

use self::systems::*;

use super::SimulationState;

pub const BULLET_SPEED: f32 = 600.0;
pub const BULLET_SIZE: f32 = 16.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bullet_fire_on_click,
                bullet_movement,
                bullet_despawn_off_screen,
                bullet_kill_enemies
            )
                .run_if(in_state(SimulationState::Running))
                .run_if(in_state(AppState::Game)),
        )
        .add_systems(OnExit(AppState::Game), despawn_all_bullets);
    }
}
