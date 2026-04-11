use bevy::prelude::*;

use crate::{
    components::{Enemy, Player},
    messages::{EnemyInProximity, PlayerDamaged},
    resources::GameState,
    settings::GameSettings,
};

pub fn process_enemy_ai(
    time: Res<Time>,
    game_state: Res<GameState>,
    settings: Res<GameSettings>,
    player_query: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    if game_state.game_over {
        return;
    }

    let player_transform = player_query.into_inner();

    for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
        if enemy.health > 0 {
            let to_player =
                player_transform.translation.truncate() - enemy_transform.translation.truncate();

            if to_player != Vec2::ZERO {
                let direction = to_player.normalize();
                let movement = direction * settings.enemy.speed * time.delta().as_secs_f32();

                enemy_transform.translation.x += movement.x;
                enemy_transform.translation.y += movement.y;
            }
        }
    }
}

pub fn check_enemy_player_proximity(
    game_state: ResMut<GameState>,
    settings: Res<GameSettings>,
    enemy_query: Query<(Entity, &Transform, &Enemy), Without<Player>>,
    player_query: Single<(&Transform, &mut Player)>,
    mut proximity_events: MessageWriter<EnemyInProximity>,
) {
    if game_state.game_over {
        return;
    }

    let (player_transform, player) = player_query.into_inner();

    if player.health <= 0 {
        return;
    }

    for (_entity, enemy_transform, _enemy) in enemy_query.iter() {
        // Calculate distance between player and enemy
        let to_player =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        let distance = to_player.length();
        // If enemy is close enough, damage the player
        if distance <= settings.enemy.damage_range {
            proximity_events.write(EnemyInProximity);
        }
    }
}

pub fn process_enemy_attack(
    mut enemy_in_proximity_events: MessageReader<EnemyInProximity>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    player_query: Single<(Entity, &Transform, &mut Player)>,
    enemy_query: Query<(Entity, &Transform, &Enemy), Without<Player>>,
    asset_server: Res<AssetServer>,
    mut player_damaged_events: MessageWriter<PlayerDamaged>,
) {
    if game_state.game_over {
        return;
    }

    let (_player_entity, _player_transform, mut player) = player_query.into_inner();

    if player.health <= 0 {
        return;
    }

    for _event in enemy_in_proximity_events.read() {
        for (_entity, _enemy_transform, _enemy) in enemy_query.iter() {
            // Apply damage every 1 second using the game state timer
            game_state.damage_timer += time.delta().as_secs_f32();

            if game_state.damage_timer >= 1.0 {
                game_state.damage_timer = 0.0;
                player.health -= 1;
                commands.spawn(AudioPlayer::new(asset_server.load("enemy_hit.ogg")));
                debug!("Player damaged! Health: {}", player.health);
                player_damaged_events.write(PlayerDamaged { _damage: 1 });
            }
        }
    }
}
