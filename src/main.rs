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
mod music;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LevelSelection};

use crate::collision::{enemy_wall_collision, player_wall_collision};
use crate::enemy::{check_enemy_player_proximity, process_enemy_ai, process_enemy_attack, process_enemy_damaged, process_enemy_death};
use crate::messages::{DamageEvent, EnemyDamaged, EnemyInProximity, EnemyKilled, PlayerDamaged};
use crate::music::on_level_spawned;
use crate::player::{check_player_moved, process_player_attack, process_player_damaged, process_player_death};
use crate::resources::{CurrentMusic, GameState, LevelFlow};
use crate::score::{on_enemy_killed, on_player_damaged, process_combo_timer, ScoreState};
use crate::settings::GameSettings;
use crate::setup::setup;
use crate::systems::{apply_ldtk_entity_blueprints, check_restart_button, process_attack_cooldowns, process_bullet_collision, process_bullet_movement, process_weapon_pickup};
use crate::ui::{setup_ui, process_ui_updates};
use crate::systems::{process_goal_interaction};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(GameSettings::default())
        .insert_resource(LevelFlow {
            index: 0,
            total: GameSettings::default().game.total_levels,
        })
        .insert_resource(GameState {
            game_over: false,
            victory: false,
            damage_timer: 0.0,
            enemies_remaining: 0,
            reached_goal: false,
        })
        .insert_resource(ScoreState::default())
        .insert_resource(CurrentMusic {
            entity: Entity::PLACEHOLDER,
            level_name: String::new(),
        })
        .add_message::<EnemyKilled>()
        .add_message::<PlayerDamaged>()
        .add_message::<EnemyDamaged>()
        .add_message::<EnemyInProximity>()
        .add_message::<DamageEvent>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (
            apply_ldtk_entity_blueprints,
            on_level_spawned,
            process_weapon_pickup,
            process_attack_cooldowns,
            check_player_moved,
            check_enemy_player_proximity,
            process_enemy_ai,
            process_enemy_attack,
            process_enemy_damaged,
            process_enemy_death,
        ))
        .add_systems(Update, (
            process_player_attack,
            process_player_damaged,
            process_player_death,
            on_enemy_killed,
            on_player_damaged,
            process_combo_timer,
            process_ui_updates,
            check_restart_button,
        ))
        .add_systems(Update,
            player_wall_collision.after(check_player_moved),
        )
        .add_systems(Update,
            enemy_wall_collision.after(process_enemy_ai),
        )
        .add_systems(Update,
            process_bullet_movement.after(process_player_attack),
        )
        .add_systems(Update,
            process_bullet_collision.after(process_bullet_movement),
        )
        .add_systems(Update, process_goal_interaction)
        .run();
}
