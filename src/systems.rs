use bevy::prelude::*;

use crate::{
    components::{AttackCooldown, Enemy, GameEntity, Player},
    resources::GameState,
    utils::restart_game,
};

pub fn update_attack_cooldowns(time: Res<Time>, mut cooldown_query: Query<&mut AttackCooldown>) {
    for mut cooldown in cooldown_query.iter_mut() {
        cooldown.0.tick(time.delta());
    }
}

pub fn cleanup_dead_entities(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    enemy_query: Query<(Entity, &Enemy), Without<Player>>,
) {
    // Clean up dead enemies and update counter
    for (entity, enemy) in enemy_query.iter() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
            game_state.enemies_remaining -= 1;
        }
    }
}

pub fn check_restart_button(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        restart_game(&mut commands, &mut game_state, all_entities, &asset_server);
    }
}
