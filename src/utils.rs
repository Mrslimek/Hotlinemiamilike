use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query, ResMut},
};
use bevy::prelude::AssetServer;

use crate::{
    components::GameEntity,
    resources::{setup_initial_state, GameState},
    setup::spawn_game_world,
};

pub fn restart_game(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    all_entities: Query<Entity, With<GameEntity>>,
    asset_server: &AssetServer,
) {
    for entity in all_entities.iter() {
        commands.entity(entity).despawn();
    }
    setup_initial_state(game_state);
    spawn_game_world(commands, asset_server);
}

