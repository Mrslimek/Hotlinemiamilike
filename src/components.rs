use bevy::prelude::*;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Damage {
    pub amount: i32,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Weapon {
    pub damage: i32,
    pub weapon_type: WeaponType,
    pub range: f32,
    pub cooldown: Timer,
}

#[derive(Clone, Debug)]
pub enum WeaponType {
    Melee { knockback: f32 },
    Ranged { speed: f32, spread: f32 },
}

#[derive(Component)]
pub struct AttackCooldown(pub Timer);

#[derive(Component)]
pub struct TextScreen;

/// AABB collider storing the half-size of the entity's bounding box.
/// Used for collision detection and resolution.
#[derive(Component)]
pub struct Collider {
    pub half_size: Vec2,
}

/// Marker component for static wall entities.
#[derive(Component, Default)]
pub struct Wall;

#[derive(Component)]
pub struct Goal;
