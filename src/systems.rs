use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LevelSelection};

use crate::{
    components::{AttackCooldown, Collider, Enemy, GameEntity, Goal, Player, Wall},
    resources::{GameState, LevelFlow},
    settings::GameSettings,
    // utils::restart_game,
};

pub fn process_attack_cooldowns(time: Res<Time>, mut cooldown_query: Query<&mut AttackCooldown>) {
    for mut cooldown in cooldown_query.iter_mut() {
        cooldown.0.tick(time.delta());
    }
}

// TODO: Refactor this to use events system, not per frame calculations
// pub fn cleanup_dead_entities(
//     mut commands: Commands,
//     mut game_state: ResMut<GameState>,
//     enemy_query: Query<(Entity, &Enemy), With<Enemy>>,
// ) {
//     // Clean up dead enemies and update counter
//     for (entity, enemy) in enemy_query.iter() {
//         if enemy.health <= 0 {
//             commands.entity(entity).despawn();
//             game_state.enemies_remaining -= 1;
//         }
//     }
// }

pub fn process_camera_movement(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}

pub fn process_goal_interaction(
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    goal_query: Query<&Transform, With<Goal>>,
    mut level_flow: ResMut<LevelFlow>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if game_state.reached_goal {
        game_state.reached_goal = false;

        level_flow.index += 1;
        if level_flow.index >= level_flow.total {
            level_flow.index = 0;
            debug!("All levels completed. Looping back to level_0.");
        } else {
            debug!("Loading level_{}...", level_flow.index);
        }

        *level_selection = LevelSelection::index(level_flow.index);
    }

    let Ok(player) = player_query.single() else {
        return;
    };

    for goal in goal_query.iter() {
        let distance = player
            .translation
            .truncate()
            .distance(goal.translation.truncate());
        if distance <= 20.0 {
            game_state.reached_goal = true;
            debug!("Goal reached!");
            break;
        }
    }
}

pub fn apply_ldtk_entity_blueprints(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<GameSettings>,
    mut query: Query<(Entity, &EntityInstance, &mut Transform), Added<EntityInstance>>,
) {
    for (entity, instance, mut transform) in query.iter_mut() {
        match instance.identifier.as_str() {
            "Player" => {
                transform.translation.z = 10.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("player.png")),
                    Player { health: 3 },
                    AttackCooldown(Timer::from_seconds(
                        settings.player.attack_cooldown,
                        TimerMode::Once,
                    )),
                    Collider {
                        half_size: Vec2::splat(8.0), // 16px sprite / 2 = 8px half-size
                    },
                ));
            }
            "Enemy" => {
                transform.translation.z = 10.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("enemy.png")),
                    Enemy { health: 1 },
                    Collider {
                        half_size: Vec2::splat(8.0), // 16px sprite / 2 = 8px half-size
                    },
                ));
            }
            "Wall" => {
                transform.translation.z = 5.0;
                commands.entity(entity).insert((
                    Sprite {
                        color: Color::srgb(0.65, 0.55, 0.40),
                        custom_size: Some(Vec2::splat(16.0)),
                        ..default()
                    },
                    Wall,
                    Collider {
                        half_size: Vec2::splat(8.0), // 16px sprite / 2 = 8px half-size
                    },
                ));
            }
            "Goal" => {
                transform.translation.z = 9.0;
                commands.entity(entity).insert((
                    Sprite {
                        color: Color::srgb(0.2, 0.9, 0.35),
                        custom_size: Some(Vec2::splat(16.0)),
                        ..default()
                    },
                    Goal,
                ));
            }
            _ => {}
        }
    }
}

pub fn check_restart_button(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut level_selection: ResMut<LevelSelection>,
    keyboard: Res<ButtonInput<KeyCode>>,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    if game_state.game_over && keyboard.just_pressed(KeyCode::KeyR) {
        debug!("Restarting level...");

        for entity in all_entities.iter() {
            commands.entity(entity).despawn();
        }

        game_state.game_over = false;
        game_state.victory = false;
        game_state.damage_timer = 0.0;
        game_state.enemies_remaining = 0;
        game_state.reached_goal = false;

        *level_selection = LevelSelection::index(0);
    }
}
