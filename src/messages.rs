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
pub struct EnemyDamaged {
    pub enemy_entity: Entity,
    pub damage: i32,
}

#[derive(Message)]
pub struct EnemyInProximity {
    pub _enemy_entity: Entity
}

#[derive(Message)]
pub struct DamageEvent {
    pub target: Entity,
    pub amount: i32,
    pub source: Entity,
}
