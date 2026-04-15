use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{
    components::{Ammo, AttackCooldown, Bullet, Damage, Enemy, Firearm, GameEntity, Health, MeleeWeapon, Player, TextScreen},
    messages::{DamageEvent, PlayerDamaged},
    resources::GameState,
    settings::GameSettings
};

pub fn check_player_moved(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    query: Single<&mut Transform, With<Player>>,
) {
    let mut transform = query.into_inner();

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    let movement = direction * settings.player.speed * time.delta().as_secs_f32();
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}

pub fn process_player_attack(
    mouse: Res<ButtonInput<MouseButton>>,
    q_windows: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    game_state: ResMut<GameState>,
    settings: Res<GameSettings>,
    mut commands: Commands,
    player_query: Single<(Entity, &Transform, &mut AttackCooldown), With<Player>>,
    melee_weapon_query: Query<&MeleeWeapon, With<Player>>,
    mut firearm_query: Query<(&Firearm, &mut Ammo), With<Player>>,
    damage_query: Query<&Damage, With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    asset_server: Res<AssetServer>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    if game_state.game_over {
        return;
    }

    let (player_entity, player_transform, mut cooldown) = player_query.into_inner();

    cooldown.0.tick(time.delta());

    if mouse.just_pressed(MouseButton::Left) && cooldown.0.is_finished() {
        cooldown.0.reset();

        commands.spawn(AudioPlayer::new(asset_server.load("player_attack.ogg")));

        let window = q_windows.into_inner();

        if let Some(cursor_pos) = window.cursor_position() {
            let world_mouse_pos = Vec2::new(
                cursor_pos.x - window.width() / 2.0,
                -(cursor_pos.y - window.height() / 2.0),
            );

            let attack_direction =
                (world_mouse_pos - player_transform.translation.truncate()).normalize_or_zero();

            // Determine weapon type and attack accordingly
            let has_firearm = firearm_query.get(player_entity).is_ok();
            let has_melee = melee_weapon_query.get(player_entity).is_ok();
            let base_damage = damage_query.get(player_entity).map(|d| d.amount).unwrap_or(1);

            if has_firearm {
                // Firearm shooting
                if let Ok((firearm, mut ammo)) = firearm_query.get_mut(player_entity) {
                    if ammo.current > 0 {
                        ammo.current -= 1;

                        // Determine number of bullets based on weapon type
                        // Shotgun shoots multiple bullets, others shoot one
                        let bullet_count = if firearm.spread > 0.25 { 5 } else { 1 };

                        for i in 0..bullet_count {
                            // Calculate spread angle
                            let spread_angle = if bullet_count > 1 {
                                // Spread bullets in a cone for shotgun
                                let cone_angle = firearm.spread;
                                let start_angle = -cone_angle / 2.0;
                                let angle_step = cone_angle / (bullet_count - 1) as f32;
                                start_angle + angle_step * i as f32
                            } else {
                                // Random spread for single bullet weapons
                                let mut rng = rand::thread_rng();
                                rng.gen_range(-firearm.spread..firearm.spread)
                            };

                            // Apply spread to direction
                            let spread_direction = if spread_angle != 0.0 {
                                let cos_angle = spread_angle.cos();
                                let sin_angle = spread_angle.sin();
                                Vec2::new(
                                    attack_direction.x * cos_angle - attack_direction.y * sin_angle,
                                    attack_direction.x * sin_angle + attack_direction.y * cos_angle
                                )
                            } else {
                                attack_direction
                            };

                            let bullet_velocity = spread_direction * firearm.bullet_speed;

                            commands.spawn((
                                Sprite {
                                    color: Color::srgb(1.0, 1.0, 0.0),
                                    custom_size: Some(Vec2::splat(4.0)),
                                    ..default()
                                },
                                Transform::from_translation(player_transform.translation),
                                Bullet {
                                    velocity: bullet_velocity,
                                    damage: firearm.damage,
                                    owner: player_entity,
                                    lifetime: Timer::from_seconds(2.0, TimerMode::Once),
                                },
                            ));
                        }
                    }
                }
            } else if has_melee {
                // Melee weapon attack
                if let Ok(melee_weapon) = melee_weapon_query.get(player_entity) {
                    for (enemy_entity, enemy_transform, _enemy) in enemy_query.iter_mut() {
                        let to_enemy = enemy_transform.translation.truncate()
                            - player_transform.translation.truncate();
                        let distance = to_enemy.length();

                        if distance <= melee_weapon.range {
                            let dot_product = to_enemy.normalize_or_zero().dot(attack_direction);

                            if dot_product > 0.0 {
                                damage_events.write(DamageEvent {
                                    target: enemy_entity,
                                    amount: melee_weapon.damage,
                                    source: player_entity
                                });
                            }
                        }
                    }
                }
            } else {
                // Fist attack (no weapon)
                for (enemy_entity, enemy_transform, _enemy) in enemy_query.iter_mut() {
                    let to_enemy = enemy_transform.translation.truncate()
                        - player_transform.translation.truncate();
                    let distance = to_enemy.length();

                    if distance <= settings.player.attack_range {
                        let dot_product = to_enemy.normalize_or_zero().dot(attack_direction);

                        if dot_product > 0.0 {
                            damage_events.write(DamageEvent {
                                target: enemy_entity,
                                amount: base_damage,
                                source: player_entity
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn process_player_death(
    mut events: MessageReader<PlayerDamaged>,
    text_screen_query: Query<&TextScreen>,
    mut game_state: ResMut<GameState>,
    player_query: Single<Entity, With<Player>>,
    mut commands: Commands,
    health_query: Query<&Health, With<Player>>,
) {
    let player_entity = player_query.into_inner();

    for _event in events.read() {
        if let Ok(health) = health_query.get(player_entity) {
            if health.current <= 0 && !game_state.victory && text_screen_query.count() == 0 {
                info!("GAME OVER! Player died.");
                game_state.game_over = true;
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

pub fn process_player_damaged (
    mut damage_events: MessageReader<DamageEvent>,
    player_query: Single<Entity, With<Player>>,
    mut player_damaged_events: MessageWriter<PlayerDamaged>,
    mut health_query: Query<&mut Health, With<Player>>,
) {
    let player_entity = player_query.into_inner();

    for event in damage_events.read() {
        if event.target == player_entity {
            if let Ok(mut health) = health_query.get_mut(event.target) {
                health.current -= event.amount;
                player_damaged_events.write(PlayerDamaged { _damage: event.amount });
            }
        }
    }
}
