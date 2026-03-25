use bevy::prelude::*;

use crate::components::Enemy;

#[derive(Resource)]
pub struct GameState {
    pub game_over: bool,
    pub victory: bool,
    pub damage_timer: f32,
    pub enemies_remaining: usize,
    pub reached_goal: bool,
}

#[derive(Resource)]
pub struct LevelFlow {
    pub total: usize,
    pub index: usize,
}

pub fn setup_initial_state(
    game_state: &mut ResMut<GameState>,
    enemy_query: Query<Entity, With<Enemy>>,
) {
    game_state.enemies_remaining = enemy_query.iter().count();
    game_state.game_over = false;
    game_state.victory = false;
    game_state.damage_timer = 0.0;
}
