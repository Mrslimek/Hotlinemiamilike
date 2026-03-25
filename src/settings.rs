use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    // UI Settings
    pub SHOW_UI: bool,
    pub DEBUG_MODE: bool,

    // Player Settings
    pub PLAYER_SPEED: f32,
    pub PLAYER_SIZE: f32,
    pub ATTACK_RANGE: f32,
    pub ATTACK_COOLDOWN: f32,

    // Enemy Settings
    pub ENEMY_SIZE: f32,
    pub ENEMY_SPEED: f32,
    pub ENEMY_DAMAGE_RANGE: f32,

    // Visual Settings
    pub GROUND_COLOR: Color,
    pub WALL_COLOR: Color,

    // Room Settings
    pub ROOM_HALF_SIZE: f32,
    pub WALL_THICKNESS: f32,
    pub DOOR_HALF_WIDTH: f32,

    // Game Settings
    pub TOTAL_LEVELS: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            SHOW_UI: true,
            DEBUG_MODE: false,

            // Player (было в constants.rs)
            PLAYER_SPEED: 200.0,
            PLAYER_SIZE: 32.0,
            ATTACK_RANGE: 50.0,
            ATTACK_COOLDOWN: 0.5,

            // Enemy (было в constants.rs)
            ENEMY_SIZE: 32.0,
            ENEMY_SPEED: 100.0,
            ENEMY_DAMAGE_RANGE: 40.0,

            // Visuals (было в constants.rs)
            GROUND_COLOR: Color::srgb(0.15, 0.15, 0.15),
            WALL_COLOR: Color::srgb(0.55, 0.45, 0.35),

            // Room (было в constants.rs)
            ROOM_HALF_SIZE: 150.0,
            WALL_THICKNESS: 12.0,
            DOOR_HALF_WIDTH: 25.0,

            // Game (было в constants.rs)
            TOTAL_LEVELS: 10,
        }
    }
}
