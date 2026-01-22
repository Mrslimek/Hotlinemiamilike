use bevy::prelude::*;

use crate::{
    components::{Enemy, GameEntity, Player, TextScreen},
    constants::{ENEMY_DAMAGE_RANGE, ENEMY_SPEED},
    resources::GameState,
    utils::restart_game,
};

pub fn enemy_ai(
    time: Res<Time>,
    game_state: Res<GameState>,
    player_query: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    // Enemies don't move if game is over
    if game_state.game_over {
        return;
    }

    let player_transform = player_query.into_inner();

    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        // Simple AI: move toward player if alive
        if enemy.health > 0 {
            let to_player =
                player_transform.translation.truncate() - enemy_transform.translation.truncate();

            if to_player != Vec2::ZERO {
                let direction = to_player.normalize();
                let movement = direction * ENEMY_SPEED * time.delta().as_secs_f32();

                enemy_transform.translation.x += movement.x;
                enemy_transform.translation.y += movement.y;
            }
        }
    }
}

pub fn enemy_damage(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    player_query: Single<(Entity, &Transform, &mut Player)>,
    enemy_query: Query<(&Transform, &Enemy), Without<Player>>,
    all_entities: Query<Entity, With<GameEntity>>,
    text_screen_query: Query<&TextScreen>,
) {
    // Don't process damage if game is over
    if game_state.game_over {
        return;
    }

    let (_player_entity, player_transform, mut player) = player_query.into_inner();

    if player.health <= 0 {
        return;
    }

    for (enemy_transform, enemy) in enemy_query.iter() {
        // Check if enemy is alive
        if enemy.health <= 0 {
            continue;
        }

        // Calculate distance between player and enemy
        let to_player =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        let distance = to_player.length();

        // If enemy is close enough, damage the player
        if distance <= ENEMY_DAMAGE_RANGE {
            // Apply damage every 1 second using the game state timer
            game_state.damage_timer += time.delta().as_secs_f32();

            if game_state.damage_timer >= 1.0 {
                game_state.damage_timer = 0.0;
                player.health -= 1;

                if player.health <= 0 && !game_state.victory && text_screen_query.count() == 0 {
                    restart_game(&mut commands, &mut game_state, all_entities);
                }
            }
        }
    }
}
