use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Commands, Query, ResMut},
};
use bevy::prelude::AssetServer;

use crate::{
    components::{Enemy, GameEntity},
    resources::{GameState, setup_initial_state},
    setup::spawn_game_world,
};

pub fn restart_game(
    commands: &mut Commands,
    game_state: &mut ResMut<GameState>,
    all_entities: Query<Entity, With<GameEntity>>,
    enemy_query: Query<Entity, With<Enemy>>,
    asset_server: &AssetServer,
) {
    for entity in all_entities.iter() {
        commands.entity(entity).despawn();
    }
    setup_initial_state(game_state, enemy_query);
    spawn_game_world(commands, asset_server);
}
