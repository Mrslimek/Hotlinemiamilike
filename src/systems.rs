use bevy::prelude::*;
use bevy_ecs_ldtk::{EntityInstance, LevelSelection};

use crate::{
    components::{Ammo, AttackCooldown, Bullet, Collider, Damage, Enemy, Firearm, GameEntity, Goal, Health, MeleeWeapon, Player, Wall, Weapon, WeaponPickup},
    messages::DamageEvent,
    resources::{GameState, LevelFlow},
    settings::GameSettings,
};

// Helper type for weapon queries
type PlayerWithWeapon = (Entity, &'static Transform);

pub fn process_attack_cooldowns(time: Res<Time>, mut cooldown_query: Query<&mut AttackCooldown>) {
    // TODO: Add some animation (dev purpose) to show this cooldown
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
    // TODO: Check why this should run every frame
    for (entity, instance, mut transform) in query.iter_mut() {
        match instance.identifier.as_str() {
            "Player" => {
                transform.translation.z = 10.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("player.png")),
                    Player,
                    Health {
                        max: 3,
                        current: 3
                    },
                    Damage {
                        amount: 1
                    },
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
                    Enemy,
                    Health {
                        max: 1,
                        current: 1
                    },
                    Damage {
                        amount: 1
                    },
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
            "WeaponBat" => {
                transform.translation.z = 8.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("weapon_bat.png")),
                    WeaponPickup,
                    // Store weapon data for pickup
                    MeleeWeapon {
                        damage: 2,
                        range: 30.0,
                        attack_speed: 0.8,
                        knockback: 50.0,
                    },
                ));
            }
            "WeaponPistol" => {
                transform.translation.z = 8.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("weapon_pistol.png")),
                    WeaponPickup,
                    Firearm {
                        damage: 1,
                        fire_rate: 0.5,
                        bullet_speed: 400.0,
                        spread: 0.1,
                    },
                    Ammo {
                        current: 6,
                        max: 6,
                    },
                ));
            }
            "WeaponShotgun" => {
                transform.translation.z = 8.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("weapon_shotgun.png")),
                    WeaponPickup,
                    Firearm {
                        damage: 1,
                        fire_rate: 1.0,
                        bullet_speed: 350.0,
                        spread: 0.3,
                    },
                    Ammo {
                        current: 2,
                        max: 2,
                    },
                ));
            }
            "WeaponMachineGun" => {
                transform.translation.z = 8.0;
                commands.entity(entity).insert((
                    Sprite::from_image(asset_server.load("weapon_machine_gun.png")),
                    WeaponPickup,
                    Firearm {
                        damage: 1,
                        fire_rate: 0.15,  // Fast fire rate
                        bullet_speed: 500.0,
                        spread: 0.2,
                    },
                    Ammo {
                        current: 20,
                        max: 20,
                    },
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

pub fn process_weapon_pickup(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    weapon_pickups: Query<(Entity, &Transform, &WeaponPickup)>,
    melee_weapons: Query<&MeleeWeapon, With<WeaponPickup>>,
    firearms: Query<(&Firearm, &Ammo), With<WeaponPickup>>,
) {
    let Ok((player_entity, player_transform)) = player_query.single() else {
        return;
    };

    for (weapon_entity, weapon_transform, _weapon_pickup) in weapon_pickups.iter() {
        let distance = player_transform.translation.truncate()
            .distance(weapon_transform.translation.truncate());

        if distance < 20.0 {
            // Remove old weapon if player has one
            commands.entity(player_entity)
                .remove::<(Weapon, MeleeWeapon, Firearm, Ammo)>();

            // Check if this is a melee weapon
            if let Ok(melee_data) = melee_weapons.get(weapon_entity) {
                commands.entity(player_entity).insert(Weapon);
                commands.entity(player_entity).insert(*melee_data);
                info!("Picked up melee weapon!");
            }
            // Check if this is a firearm
            else if let Ok((firearm_data, ammo_data)) = firearms.get(weapon_entity) {
                commands.entity(player_entity).insert(Weapon);
                commands.entity(player_entity).insert(*firearm_data);
                commands.entity(player_entity).insert(*ammo_data);
                info!("Picked up firearm!");
            }

            // Remove the pickup from the world
            commands.entity(weapon_entity).despawn();
        }
    }
}

pub fn process_bullet_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut bullet_query: Query<(Entity, &mut Transform, &mut Bullet)>,
) {
    for (bullet_entity, mut transform, mut bullet) in bullet_query.iter_mut() {
        // Move bullet
        transform.translation.x += bullet.velocity.x * time.delta().as_secs_f32();
        transform.translation.y += bullet.velocity.y * time.delta().as_secs_f32();

        // Update lifetime
        bullet.lifetime.tick(time.delta());

        // Despawn if lifetime expired
        if bullet.lifetime.is_finished() {
            commands.entity(bullet_entity).despawn();
        }
    }
}

pub fn process_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    enemy_query: Query<(Entity, &Transform, &Enemy)>,
    wall_query: Query<(Entity, &Transform, &Wall)>,
    mut damage_events: MessageWriter<DamageEvent>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        // Check collision with enemies
        for (enemy_entity, enemy_transform, _enemy) in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = bullet_pos.distance(enemy_pos);

            if distance < 16.0 { // Enemy size
                damage_events.write(DamageEvent {
                    target: enemy_entity,
                    amount: bullet.damage,
                    source: bullet.owner,
                });
                commands.entity(bullet_entity).despawn();
                return;
            }
        }

        // Check collision with walls
        for (_wall_entity, wall_transform, _wall) in wall_query.iter() {
            let wall_pos = wall_transform.translation.truncate();
            let distance = bullet_pos.distance(wall_pos);

            if distance < 16.0 { // Wall size
                commands.entity(bullet_entity).despawn();
                return;
            }
        }
    }
}
