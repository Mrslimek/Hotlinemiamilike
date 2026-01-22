use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 200.0;
pub const PLAYER_SIZE: f32 = 32.0;
pub const ENEMY_SIZE: f32 = 32.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const ATTACK_RANGE: f32 = 50.0;
pub const ATTACK_COOLDOWN: f32 = 0.5;
pub const ENEMY_DAMAGE_RANGE: f32 = 40.0;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.05, 0.05, 0.05);
pub const PLAYER_COLOR: Color = Color::srgb(0.95, 0.3, 0.2); // pink/red
pub const ENEMY_COLOR: Color = Color::srgb(0.3, 0.3, 0.95); // Blue
pub const GROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

//NOTE: This will be state for the first level
pub const ENEMY_COUNT: i32 = 5;
