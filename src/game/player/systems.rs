use bevy::{audio::Volume, prelude::*, window::PrimaryWindow};

use crate::game::{enemy::{components::Enemy, ENEMY_SIZE}, star::{components::Star, STAR_SIZE}, GameOver};
use super::components::Player;
use crate::game::score::resources::Score;

pub const PLAYER_SIZE: f32 = 64.0; // This is the size of the player sprite
pub const PLAYER_SPEED: f32 = 500.0;


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.single();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>
) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}


pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        let x_min = half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);

            let collision_distance = (PLAYER_SIZE / 2.0) + (ENEMY_SIZE / 2.0);

            if distance < collision_distance {
                commands.entity(player_entity).despawn();

                let sound_effect: Handle<AudioSource> =
                    asset_server.load("audio/explosionCrunch_000.ogg");

                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings {
                        volume: Volume::new(0.1),
                        ..default()
                    },
                    ..default()
                });

                game_over_event_writer.send(GameOver {
                    score: score.value
                });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single_mut() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            let collision_distance = (PLAYER_SIZE + STAR_SIZE) / 2.0;

            if distance < collision_distance {
                commands.entity(star_entity).despawn();

                let sound_effect: Handle<AudioSource> = asset_server.load("audio/pluck_002.ogg");

                commands.spawn(AudioBundle {
                    source: sound_effect,
                    settings: PlaybackSettings {
                        volume: Volume::new(0.1),
                        ..default()
                    },
                    ..default()
                });

                score.value += 1;
            }
        }
    }
}