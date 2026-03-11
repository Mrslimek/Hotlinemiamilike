use bevy::prelude::*;

use crate::components::{AttackCooldown, Collider, Enemy, GameEntity, Player, Wall};
use crate::constants::{
    ATTACK_COOLDOWN, DOOR_HALF_WIDTH, ENEMY_SIZE, GROUND_COLOR, PLAYER_SIZE, ROOM_HALF_SIZE,
    WALL_COLOR, WALL_THICKNESS,
};

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
        Collider {
            half_size: Vec2::splat(PLAYER_SIZE / 2.0),
        },
        GameEntity,
    ));

    // Spawn enemies
    spawn_enemies(commands, asset_server);

    // Spawn room with 4 doorways
    spawn_room(commands);
}

fn spawn_enemies(commands: &mut Commands, asset_server: &AssetServer) {
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
            Collider {
                half_size: Vec2::splat(ENEMY_SIZE / 2.0),
            },
            GameEntity,
        ));
    }
}

/// Spawns a rectangular room in the center of the level.
/// Each of the 4 sides has a doorway in the middle, created by splitting
/// each wall into 2 segments with a gap between them.
fn spawn_room(commands: &mut Commands) {
    // Half-length of each wall segment (on either side of the door)
    let piece_half_x = (ROOM_HALF_SIZE - DOOR_HALF_WIDTH) / 2.0;
    let piece_half_y = (ROOM_HALF_SIZE - DOOR_HALF_WIDTH) / 2.0;

    // Size of horizontal wall pieces (top/bottom walls)
    let h_half_size = Vec2::new(piece_half_x, WALL_THICKNESS / 2.0);

    // Size of vertical wall pieces (left/right walls)
    let v_half_size = Vec2::new(WALL_THICKNESS / 2.0, piece_half_y);

    // Distance from center to the center of each wall piece
    let h_center_offset = DOOR_HALF_WIDTH + piece_half_x;
    let v_center_offset = DOOR_HALF_WIDTH + piece_half_y;

    // Each tuple: (center position, half_size for Collider and Sprite)
    let wall_segments: &[(Vec2, Vec2)] = &[
        // Top wall — left piece
        (Vec2::new(-h_center_offset, ROOM_HALF_SIZE), h_half_size),
        // Top wall — right piece
        (Vec2::new(h_center_offset, ROOM_HALF_SIZE), h_half_size),
        // Bottom wall — left piece
        (Vec2::new(-h_center_offset, -ROOM_HALF_SIZE), h_half_size),
        // Bottom wall — right piece
        (Vec2::new(h_center_offset, -ROOM_HALF_SIZE), h_half_size),
        // Left wall — top piece
        (Vec2::new(-ROOM_HALF_SIZE, v_center_offset), v_half_size),
        // Left wall — bottom piece
        (Vec2::new(-ROOM_HALF_SIZE, -v_center_offset), v_half_size),
        // Right wall — top piece
        (Vec2::new(ROOM_HALF_SIZE, v_center_offset), v_half_size),
        // Right wall — bottom piece
        (Vec2::new(ROOM_HALF_SIZE, -v_center_offset), v_half_size),
    ];

    for (position, half_size) in wall_segments {
        commands.spawn((
            Sprite {
                color: WALL_COLOR,
                custom_size: Some(*half_size * 2.0),
                ..default()
            },
            Transform::from_xyz(position.x, position.y, 0.0),
            Wall,
            Collider {
                half_size: *half_size,
            },
            GameEntity,
        ));
    }
}

// System wrapper — only for Bevy startup
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_game_world(&mut commands, &asset_server);
}
