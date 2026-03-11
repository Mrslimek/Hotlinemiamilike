use bevy::prelude::*;

#[derive(Component)]
pub struct GameEntity;

#[derive(Component)]
pub struct Player {
    pub health: i32,
}

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
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
#[derive(Component)]
pub struct Wall;
