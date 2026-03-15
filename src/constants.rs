use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 32.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const ATTACK_RANGE: f32 = 50.0;
pub const ATTACK_COOLDOWN: f32 = 0.5;
pub const ENEMY_DAMAGE_RANGE: f32 = 40.0;

pub const GROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const WALL_COLOR: Color = Color::srgb(0.55, 0.45, 0.35);

// Room (box with 4 entrances) constants
pub const ROOM_HALF_SIZE: f32 = 150.0;
pub const WALL_THICKNESS: f32 = 12.0;
// Half-width of each doorway opening
pub const DOOR_HALF_WIDTH: f32 = 25.0;
