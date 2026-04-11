use std::time::Duration;

use bevy::prelude::*;

use crate::messages::{EnemyKilled, PlayerDamaged};

#[derive(Resource)]
pub struct ScoreState {
    pub current_score: u32,
    pub combo_multiplier: f32,
    pub combo_timer: Timer,
    pub max_combo: f32,
    pub enemies_killed: u32,
    pub room_time: f32,
}

impl ScoreState {
    fn update(&mut self, delta: Duration) {
        if self.combo_multiplier > 1.0 {
            self.combo_timer.tick(delta);

            if self.combo_timer.is_finished() {
                self.combo_multiplier = 1.0;
            }
        }

        self.room_time += delta.as_secs_f32();
    }

    fn on_enemy_kill(&mut self) {
        let points = (100.0 * self.combo_multiplier) as u32;
        self.current_score += points;

        self.combo_multiplier = (self.combo_multiplier + 0.5).min(3.0);

        if self.combo_multiplier > self.max_combo {
            self.max_combo = self.combo_multiplier;
        }
        self.combo_timer.reset();
        self.enemies_killed += 1;
    }

    fn reset_combo(&mut self) {
        self.combo_multiplier = 1.0;
    }

    // pub fn reset_room(&mut self) {
    //     self.current_score = 0;
    //     self.combo_multiplier = 1.0;
    //     self.max_combo = 1.0;
    //     self.enemies_killed = 0;
    //     self.room_time = 0.0;
    //     self.combo_timer.reset();
    // }
}

impl Default for ScoreState {
    fn default() -> Self {
        Self {
            current_score: 0,
            combo_multiplier: 1.0,
            combo_timer: Timer::from_seconds(3.0, TimerMode::Once),
            max_combo: 1.0,
            enemies_killed: 0,
            room_time: 0.0,
        }
    }
}

pub fn on_enemy_killed(
    mut events: MessageReader<EnemyKilled>,
    mut score: ResMut<ScoreState>,
) {
    for _event in events.read() {
        score.on_enemy_kill();
    }
}

pub fn on_player_damaged(
    mut events: MessageReader<PlayerDamaged>,
    mut score: ResMut<ScoreState>,
) {
    for _event in events.read() {
        score.reset_combo();
    }
}

pub fn process_combo_timer(
    mut score: ResMut<ScoreState>,
    time: Res<Time>,
) {
    score.update(time.delta());
}
