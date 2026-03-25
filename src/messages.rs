use bevy::prelude::*;

#[derive(Message)]
pub struct EnemyKilled {
    pub position: Vec2,
}

#[derive(Message)]
pub struct PlayerDamaged {
    pub damage: i32,
}
