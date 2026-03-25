use bevy::prelude::*;

use crate::{
    components::{Enemy, GameEntity, Player, TextScreen},
    messages::{EnemyKilled, PlayerDamaged},
    resources::GameState,
    settings::GameSettings,
};

pub fn enemy_ai(
    time: Res<Time>,
    game_state: Res<GameState>,
    settings: Res<GameSettings>,
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
                let movement = direction * settings.ENEMY_SPEED * time.delta().as_secs_f32();

                enemy_transform.translation.x += movement.x;
                enemy_transform.translation.y += movement.y;
            }
        }
    }
}

pub fn enemy_damage(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    settings: Res<GameSettings>,
    mut commands: Commands,
    player_query: Single<(Entity, &Transform, &mut Player)>,
    enemy_query: Query<(Entity, &Transform, &Enemy), Without<Player>>,
    all_entities: Query<Entity, With<GameEntity>>,
    text_screen_query: Query<&TextScreen>,
    asset_server: Res<AssetServer>,
    mut enemy_killed_events: MessageWriter<EnemyKilled>,
    mut player_damaged_events: MessageWriter<PlayerDamaged>,
) {
    // Don't process damage if game is over
    if game_state.game_over {
        return;
    }

    let (_player_entity, player_transform, mut player) = player_query.into_inner();

    if player.health <= 0 {
        return;
    }

    for (entity, enemy_transform, enemy) in enemy_query.iter() {
        // Check if enemy is alive
        if enemy.health <= 0 {
            // Отправить событие ПЕРЕД despawn
            enemy_killed_events.write(EnemyKilled {
                position: enemy_transform.translation.truncate(),
            });

            commands.entity(entity).despawn();
            continue;  // Пропускаем удалённого врага
        }

        // Calculate distance between player and enemy
        let to_player =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        let distance = to_player.length();

        // If enemy is close enough, damage the player
        if distance <= settings.ENEMY_DAMAGE_RANGE {
            // Apply damage every 1 second using the game state timer
            game_state.damage_timer += time.delta().as_secs_f32();

            if game_state.damage_timer >= 1.0 {
                game_state.damage_timer = 0.0;
                player.health -= 1;

                info!("Player damaged! Health: {}", player.health);

                // Отправить message для сброса комбо
                player_damaged_events.write(PlayerDamaged { damage: 1 });

                // Play hit sound when enemy damages player
                commands.spawn(AudioPlayer::new(asset_server.load("enemy_hit.ogg")));

                if player.health <= 0 && !game_state.victory && text_screen_query.count() == 0 {
                    info!("GAME OVER! Player died.");

                    // Установить game_over - это заблокирует все movement/attack системы
                    game_state.game_over = true;

                    // Показать GAME OVER экран
                    commands
                        .spawn((
                            Node {
                                height: percent(100),
                                width: percent(100),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::BLACK),
                            GameEntity,
                        ))
                        .with_child((
                            Text::new("GAME OVER\n\nPress R to restart"),
                            TextFont {
                                font_size: 48.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.0, 0.0)),
                        ));
                }
            }
        }
    }
}
