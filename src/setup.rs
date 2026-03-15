use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkProjectHandle, LdtkWorldBundle};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: asset_server.load("levels/HotlineMiamiLikeWorld.ldtk"),
        },
        ..Default::default()
    });
}
