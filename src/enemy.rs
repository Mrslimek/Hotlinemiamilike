use bevy::prelude::*;

use crate::{
    components::{Damage, Enemy, Health, Player},
    messages::{DamageEvent, EnemyDamaged, EnemyInProximity, EnemyKilled},
    resources::GameState,
    settings::GameSettings,
};

pub fn process_enemy_ai(
    time: Res<Time>,
    game_state: Res<GameState>,
    settings: Res<GameSettings>,
    player_query: Single<&Transform, With<Player>>,
    mut enemy_query: Query<(Entity, &mut Transform), Without<Player>>,
    mut health_query: Query<&mut Health, With<Enemy>>,
) {
    if game_state.game_over {
        return;
    }

    let player_transform = player_query.into_inner();

    for (enemy_entity, mut enemy_transform) in enemy_query.iter_mut() {
        if let Ok(health) = health_query.get_mut(enemy_entity) {
            if health.current > 0 {
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
}

pub fn check_enemy_player_proximity(
    game_state: ResMut<GameState>,
    settings: Res<GameSettings>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    player_query: Single<(&Transform, &mut Player)>,
    mut proximity_events: MessageWriter<EnemyInProximity>,
) {
    if game_state.game_over {
        return;
    }

    let (player_transform, _player) = player_query.into_inner();

    for (enemy_entity, enemy_transform) in enemy_query.iter() {
        let to_player =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        let distance = to_player.length();

        if distance <= settings.enemy.damage_range {
            proximity_events.write(EnemyInProximity{
                _enemy_entity: enemy_entity
            });
        }
    }
}

pub fn process_enemy_attack(
    mut enemy_in_proximity_events: MessageReader<EnemyInProximity>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<(Entity, &Transform, &Enemy, &Damage), Without<Player>>,
    asset_server: Res<AssetServer>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    if game_state.game_over {
        return;
    }

    let mut player_entities = player_query.iter();
    let Some(player_entity) = player_entities.next() else {
        return;
    };

    let events: Vec<_> = enemy_in_proximity_events.read().collect();

    if !events.is_empty() {
        game_state.damage_timer += time.delta().as_secs_f32();

        if game_state.damage_timer >= 1.0 {
            game_state.damage_timer = 0.0;

            for event in events.iter() {
                if let Ok((_entity, _transform, _enemy, damage)) = enemy_query.get(event._enemy_entity) {
                    commands.spawn(AudioPlayer::new(asset_server.load("enemy_hit.ogg")));

                    damage_events.write(DamageEvent {
                        target: player_entity,
                        amount: damage.amount,
                        source: event._enemy_entity
                    });

                    break;
                }
            }
        }
    }
}

pub fn process_enemy_death(
    mut events: MessageReader<EnemyDamaged>,
    mut enemy_killed_events: MessageWriter<EnemyKilled>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
    health_query: Query<&Health, With<Enemy>>,
) {
    for event in events.read() {
        if let Ok((enemy_entity, enemy_transform)) = enemy_query.get(event.enemy_entity) {
            if let Ok(health) = health_query.get(enemy_entity) {
                if health.current <= 0 {
                    enemy_killed_events.write(EnemyKilled {
                        _position: enemy_transform.translation.truncate(),
                    });
                    commands.entity(enemy_entity).despawn();
                    debug!("Enemy {:?} killed!", enemy_entity);
                }
            }
        }
    }
}

pub fn process_enemy_damaged (
    mut damage_events: MessageReader<DamageEvent>,
    mut enemy_damaged_events: MessageWriter<EnemyDamaged>,
    mut health_query: Query<&mut Health, With<Enemy>>,
) {
    for event in damage_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            health.current -= event.amount;
            enemy_damaged_events.write(EnemyDamaged {
                enemy_entity: event.target,
                damage: event.amount,
            });
        }
    }
}
