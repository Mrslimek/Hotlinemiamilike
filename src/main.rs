mod collision;
mod components;
mod enemy;
mod player;
mod resources;
mod setup;
mod systems;
mod ui;
mod utils;
mod messages;
mod score;
mod settings;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

use crate::collision::{enemy_wall_collision, player_wall_collision};
use crate::enemy::{enemy_ai, enemy_damage};
use crate::messages::{EnemyKilled, PlayerDamaged};
use crate::player::{player_attack, player_movement};
use crate::resources::{GameState, LevelFlow};
use crate::score::{on_enemy_killed, on_player_damaged, update_combo_timer, ScoreState};
use crate::settings::GameSettings;
use crate::setup::setup;
use crate::systems::{apply_ldtk_entity_blueprints, camera_follow_player, check_restart_button, update_attack_cooldowns};
use crate::ui::{setup_ui, update_ui};
// use crate::systems::{check_restart_button, cleanup_dead_entities};
// use crate::systems::{goal_interaction, advance_level_on_goal};
// use crate::ui::check_game_state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(GameSettings::default())
        .insert_resource(LevelFlow {
            index: 0,
            total: GameSettings::default().TOTAL_LEVELS,
        })
        .insert_resource(GameState {
            game_over: false,
            victory: false,
            damage_timer: 0.0,
            enemies_remaining: 0,
            reached_goal: false,
        })
        .insert_resource(ScoreState::default())
        .add_event::<EnemyKilled>()
        .add_event::<PlayerDamaged>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                // КРИТИЧНО: Применяет LDtk entity blueprints (создаёт игрока, врагов, стены)
                apply_ldtk_entity_blueprints,
                // КРИТИЧНО: Камера следует за игроком
                camera_follow_player,
                // Важно: Обновляет кулдауны атаки
                update_attack_cooldowns,
                // Игровые системы
                player_movement,
                enemy_ai,
                enemy_damage,
                player_attack,
                // Score системы
                on_enemy_killed,
                on_player_damaged,
                update_combo_timer,
                // UI
                update_ui,
                // Рестарт на R
                check_restart_button,
                // Коллизия (после движения)
                player_wall_collision.after(player_movement),
                enemy_wall_collision.after(enemy_ai),
                // TODO: Level transition (позже)
                // goal_interaction,
                // advance_level_on_goal,
            ),
        )
        .run();
}
