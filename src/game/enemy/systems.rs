use bevy::{audio::Volume, prelude::*, window::PrimaryWindow};
use rand::random;

use super::components::*;
use super::resources::*;
use super::ENEMY_SIZE;
use super::ENEMY_SPEED;
use super::NUMBER_OF_ENEMIES;


pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>
) {
    for enemy in enemy_query.iter() {
        commands.entity(enemy).despawn()
    }
}


pub fn enemy_movement(mut enemy_query: Query<(&Enemy, &mut Transform)>, time: Res<Time>) {
    for (enemy, mut transform) in enemy_query.iter_mut() {
        let direction: Vec3 = enemy.direction.extend(0.0);

        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.single();

    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let mut direction_changed = false;

        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }

        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        enemy.direction = enemy.direction.normalize();

        if direction_changed {
            let sound_effect_1: Handle<AudioSource> = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2: Handle<AudioSource> = asset_server.load("audio/pluck_002.ogg");

            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };

            commands.spawn(AudioBundle {
                source: sound_effect,
                settings: PlaybackSettings {
                    volume: Volume::new(0.1),
                    ..default()
                },
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let half_enemy_size = ENEMY_SIZE / 2.0;

    let x_min = half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

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

        transform.translation = translation;
    }
}


pub fn tick_enemy_spawn_timer(
    mut timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>
) {
    timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    timer: Res<EnemySpawnTimer>,
) {
    if timer.timer.finished() {
        let window = window_query.single();

        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}