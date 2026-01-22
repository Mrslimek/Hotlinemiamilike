use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query, ResMut},
};

use crate::{
    components::GameEntity,
    resources::{GameState, setup_initial_state},
    setup::spawn_game_world,
};

pub fn restart_game(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    all_entities: Query<Entity, With<GameEntity>>,
) {
    for entity in all_entities.iter() {
        commands.entity(entity).despawn();
    }
    setup_initial_state(game_state);
    spawn_game_world(commands);
}
