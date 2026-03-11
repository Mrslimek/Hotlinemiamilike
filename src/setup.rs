use bevy::prelude::*;

use crate::components::{AttackCooldown, Enemy, GameEntity, Player};
use crate::constants::{
    ATTACK_COOLDOWN, ENEMY_SIZE, GROUND_COLOR, PLAYER_SIZE,
};

// Pure function - can be called from anywhere with &mut Commands and AssetServer
pub fn spawn_game_world(commands: &mut Commands, asset_server: &AssetServer) {
    // Camera
    commands.spawn((Camera2d, GameEntity));

    // Ground/floor
    commands.spawn((
        Sprite {
            color: GROUND_COLOR,
            custom_size: Some(Vec2::new(800.0, 600.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0),
        GameEntity,
    ));

    // Player with player.png sprite
    commands.spawn((
        Sprite::from_image(asset_server.load("player.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player { health: 3 },
        AttackCooldown(Timer::from_seconds(ATTACK_COOLDOWN, TimerMode::Once)),
        GameEntity,
    ));

    // Spawn enemies
    spawn_enemies(commands, asset_server);
}

fn spawn_enemies(commands: &mut Commands, asset_server: &AssetServer) {
    // Practice with loops and arrays
    let enemy_positions = [
        Vec2::new(200.0, 150.0),
        Vec2::new(-200.0, -150.0),
        Vec2::new(200.0, -150.0),
        Vec2::new(-200.0, 150.0),
        Vec2::new(0.0, 250.0),
    ];

    for position in enemy_positions {
        commands.spawn((
            Sprite::from_image(asset_server.load("enemy.png")),
            Transform::from_xyz(position.x, position.y, 0.0),
            Enemy { health: 1 },
            GameEntity,
        ));
    }
}

// System wrapper - only for Bevy startup
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_game_world(&mut commands, &asset_server);
}
