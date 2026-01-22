use bevy::prelude::*;

use crate::constants::ENEMY_COUNT;

#[derive(Resource)]
pub struct GameState {
    pub enemies_remaining: i32,
    pub game_over: bool,
    pub victory: bool,
    pub damage_timer: f32,
}

pub fn setup_initial_state(game_state: &mut ResMut<GameState>) {
    game_state.enemies_remaining = ENEMY_COUNT;
    game_state.game_over = false;
    game_state.victory = false;
    game_state.damage_timer = 0.0;
}
