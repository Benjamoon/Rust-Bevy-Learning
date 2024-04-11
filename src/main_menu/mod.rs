use bevy::prelude::*;

use crate::AppState;

use self::systems::{interactions::{interact_with_play_button, interact_with_quit_button}, layout::{despawn_main_menu, spawn_main_menu}};

mod components;
mod styles;
mod systems;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app

        // OnEnter State
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        
        // OnExit State
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)

        // Button Interactions
        .add_systems(Update, (
            interact_with_play_button,
            interact_with_quit_button
        ))
        
        ;
    }
}