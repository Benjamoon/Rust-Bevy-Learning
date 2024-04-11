use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use super::{components::Star, resources::StarSpawnTimer, NUMBER_OF_STARS};



pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    for _ in 0..NUMBER_OF_STARS {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn despawn_stars(
    mut commands: Commands,
    star_query: Query<Entity, With<Star>>
) {
    for star in star_query.iter() {
        commands.entity(star).despawn()
    }
}


pub fn tick_star_spawn_timer(
    mut timer: ResMut<StarSpawnTimer>,
    time: Res<Time>
) {
    timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    timer: Res<StarSpawnTimer>,
) {
    if timer.timer.finished() {
        let window = window_query.single();

        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}