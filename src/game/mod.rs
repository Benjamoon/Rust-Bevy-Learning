use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod score;
pub mod star;
mod systems;
mod ui;
mod bullet;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use ui::GameUIPlugin;
use bullet::BulletPlugin;
use systems::*;

use crate::{events::GameOver, AppState};



pub struct GamePlugin;

// The game plugin
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((
                EnemyPlugin,
                PlayerPlugin,
                ScorePlugin,
                StarPlugin,
                GameUIPlugin,
                BulletPlugin
            ))

            // On enter
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            
            // On Exit
            .add_systems(OnExit(AppState::Game), resume_simulation)

            // Systems
            .add_systems(Update, toggle_simulation.run_if(
                in_state(
                    AppState::Game
                )
            ));
    }
}

#[derive(States, Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum SimulationState {
    #[default] Running,
    Paused
}