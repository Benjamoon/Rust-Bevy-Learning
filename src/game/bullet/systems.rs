use std::f32::consts::PI;

use bevy::{
    input::mouse::MouseButtonInput, prelude::*, render::view::window, scene::ron::de,
    window::PrimaryWindow,
};

use crate::game::{
    bullet,
    enemy::{components::Enemy, ENEMY_SIZE},
    player::components::Player,
};

use super::{components::Bullet, BULLET_SIZE, BULLET_SPEED};

pub fn bullet_fire_on_click(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Ok((camera, camera_transform)) = camera_query.get_single() {
            if let Ok(player_tranform) = player_query.get_single() {
                if let Ok(window) = window_query.get_single() {
                    if let Some(world_position) = window
                        .cursor_position()
                        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                        .map(|ray| ray.origin.truncate())
                    {
                        let player_position_2d = player_tranform.translation.truncate();

                        let direction = world_position - player_position_2d;
                        let angle = direction.y.atan2(direction.x);

                        let mut bullet_transform = player_tranform.clone();
                        let offset = PI / 2.0;
                        bullet_transform.rotation = Quat::from_rotation_z(angle - offset);

                        commands.spawn((
                            SpriteBundle {
                                texture: asset_server.load("sprites/laser_bullet.png"),
                                transform: bullet_transform,
                                ..default()
                            },
                            Bullet {
                                direction: direction.normalize(),
                            },
                        ));

                        println!("Shot!")
                    }
                } else {
                    println!("Couldnt get cursor position on shoot!")
                }
            }
        }
    }
}

pub fn despawn_all_bullets(mut commands: Commands, bullets_query: Query<Entity, With<Bullet>>) {
    for bullet in bullets_query.iter() {
        commands.entity(bullet).despawn()
    }
}

pub fn bullet_despawn_off_screen(
    mut commands: Commands,
    bullets_query: Query<(&Transform, Entity), With<Bullet>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    for (bullet_transform, ent) in bullets_query.iter() {
        let translation = bullet_transform.translation;

        if translation.x > window.width() {
            commands.entity(ent).despawn();
            println!("Despawning bullet!");
        }

        if translation.y > window.height() {
            commands.entity(ent).despawn();
            println!("Despawning bullet!");
        }

        if translation.x < 0.0 {
            commands.entity(ent).despawn();
            println!("Despawning bullet!");
        }

        if translation.y < 0.0 {
            commands.entity(ent).despawn();
            println!("Despawning bullet!");
        }
    }
}

pub fn bullet_movement(
    mut bullets_query: Query<(&Bullet, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    for (bullet, mut bullet_transform) in bullets_query.iter_mut() {
        let direction = bullet.direction.extend(0.0);
        let mut position = bullet_transform.translation;

        position += direction * BULLET_SPEED * time.delta_seconds();

        bullet_transform.translation = position;
    }
}

pub fn bullet_kill_enemies(
    mut commands: Commands,
    bullets_query: Query<(Entity, &Transform), With<Bullet>>,
    enemys_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_ent, bullet_transform) in bullets_query.iter() {
        let bullet_min_x = bullet_transform.translation.x - BULLET_SIZE;
        let bullet_max_x = bullet_transform.translation.x + BULLET_SIZE;
        let bullet_min_y = bullet_transform.translation.y - BULLET_SIZE;
        let bullet_max_y = bullet_transform.translation.y + BULLET_SIZE;
    
        for (enemy_ent, enemy_transform) in enemys_query.iter() {
            let enemy_min_x = enemy_transform.translation.x - ENEMY_SIZE;
            let enemy_max_x = enemy_transform.translation.x + ENEMY_SIZE;
            let enemy_min_y = enemy_transform.translation.y - ENEMY_SIZE;
            let enemy_max_y = enemy_transform.translation.y + ENEMY_SIZE;
    
            // Check if the bullet and enemy bounding boxes overlap
            if bullet_min_x < enemy_max_x && bullet_max_x > enemy_min_x &&
               bullet_min_y < enemy_max_y && bullet_max_y > enemy_min_y {
                // Collision detected
                println!("Collision detected between bullet {:?} and enemy {:?}", bullet_ent, enemy_ent);

                commands.entity(enemy_ent).despawn();
                commands.entity(bullet_ent).despawn();
            }
        }
    }
}
