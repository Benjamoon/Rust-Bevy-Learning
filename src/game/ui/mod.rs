use bevy::prelude::*;

use self::{game_over_menu::GameOverMenuPlugin, hud::HudPlugin, pause_menu::PauseMenuPlugin};

mod hud;
mod game_over_menu;
mod pause_menu;

pub struct GameUIPlugin;

impl Plugin for GameUIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HudPlugin,
                GameOverMenuPlugin,
                PauseMenuPlugin
            ))
            
            ;
    }
}