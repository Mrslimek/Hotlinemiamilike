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

// ========== WEAPON SYSTEM ==========

/// Marker that player has a weapon
#[derive(Component)]
pub struct Weapon;

#[derive(Component, Clone, Copy)]
pub struct MeleeWeapon {
    pub damage: i32,
    pub range: f32,
    pub attack_speed: f32,
    pub knockback: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Firearm {
    pub damage: i32,
    pub fire_rate: f32,
    pub bullet_speed: f32,
    pub spread: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Ammo {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct Bullet {
    pub velocity: Vec2,
    pub damage: i32,
    pub owner: Entity, // Player or enemy who shot it
    pub lifetime: Timer,
}

/// Marker for weapon pickup items in LDtk
#[derive(Component)]
pub struct WeaponPickup;

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
