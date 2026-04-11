use bevy::prelude::*;

#[derive(Resource)]
pub struct GameSettings {
    pub ui: UiSettings,
    pub player: PlayerSettings,
    pub enemy: EnemySettings,
    pub game: GameConfigSettings,
}

#[derive(Clone, Debug)]
pub struct UiSettings {
    pub show_ui: bool,
}

#[derive(Clone, Debug)]
pub struct PlayerSettings {
    pub speed: f32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
}

#[derive(Clone, Debug)]
pub struct EnemySettings {
    pub speed: f32,
    pub damage_range: f32,
}

#[derive(Clone, Debug)]
pub struct GameConfigSettings {
    pub total_levels: usize,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            ui: UiSettings {
                show_ui: true,
            },
            player: PlayerSettings {
                speed: 200.0,
                attack_range: 50.0,
                attack_cooldown: 0.5,
            },
            enemy: EnemySettings {
                speed: 100.0,
                damage_range: 40.0,
            },
            game: GameConfigSettings {
                total_levels: 10,
            },
        }
    }
}
