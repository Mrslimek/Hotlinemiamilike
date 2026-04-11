use bevy::prelude::*;

#[derive(Message)]
pub struct EnemyKilled {
    pub _position: Vec2,
}

#[derive(Message)]
pub struct PlayerDamaged {
    pub _damage: i32,
}

#[derive(Message)]
pub struct EnemyInProximity;
