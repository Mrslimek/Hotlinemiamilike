use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkProjectHandle, LdtkWorldBundle};

#[derive(Component)]
pub struct MainCamera;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Transform::default(), MainCamera));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: asset_server.load("levels/HotlineMiamiLikeWorld.ldtk"),
        },
        ..Default::default()
    });
}
