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
