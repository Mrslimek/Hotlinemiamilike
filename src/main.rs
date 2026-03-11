mod collision;
mod components;
mod constants;
mod enemy;
mod player;
mod resources;
mod setup;
mod systems;
mod ui;
mod utils;

use bevy::prelude::*;

use crate::collision::{enemy_wall_collision, player_wall_collision};
use crate::constants::{BACKGROUND_COLOR, ENEMY_COUNT};
use crate::enemy::{enemy_ai, enemy_damage};
use crate::player::{player_attack, player_movement};
use crate::resources::GameState;
use crate::setup::setup;
use crate::systems::{check_restart_button, cleanup_dead_entities, update_attack_cooldowns};
use crate::ui::check_game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GameState {
            enemies_remaining: ENEMY_COUNT,
            game_over: false,
            victory: false,
            damage_timer: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                player_movement,
                player_attack,
                enemy_ai,
                enemy_damage,
                update_attack_cooldowns,
                cleanup_dead_entities,
                check_restart_button,
                check_game_state,
                // Collision resolution runs after movement systems
                player_wall_collision.after(player_movement),
                enemy_wall_collision.after(enemy_ai),
            ),
        )
        .run();
}
