use bevy::prelude::*;

use crate::GameOver;
use super::resources::{HighScores, Score};

pub fn insert_score(
    mut commands: Commands
) {
    commands.init_resource::<Score>()
}

pub fn remove_score(
    mut commands: Commands
) {
    commands.remove_resource::<Score>()
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value);
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
        high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1));
    }
}

pub fn high_scores_updated(
    high_scores: Res<HighScores>
) {
    if high_scores.is_changed() {
        println!("High Scores:");
        for (i, (name, score)) in high_scores.scores.iter().enumerate() {
            println!("{}. {}: {}", i + 1, name, score);
        }
    }
}