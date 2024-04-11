// Continue at
// https://youtu.be/iW19V3a96tY?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd&t=1344



pub mod events;
mod game;
mod main_menu;
mod systems;

use events::*;
use systems::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_event::<GameOver>()
        .init_state::<AppState>()
        .add_plugins((DefaultPlugins, GamePlugin, MainMenuPlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                exit_game,
                handle_game_over,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        )
        .run();
}

#[derive(States, Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
