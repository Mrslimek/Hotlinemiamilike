use bevy::prelude::*;
use bevy_ecs_ldtk::{
    assets::{LevelMetadataAccessor, LdtkProject},
    LevelEvent,
    LdtkProjectHandle,
};

use crate::resources::CurrentMusic;
use crate::utils::get_level_music;

pub fn on_level_spawned(
    mut events: MessageReader<LevelEvent>,
    project_handles: Query<&LdtkProjectHandle>,
    ldtk_assets: Res<Assets<LdtkProject>>,
    asset_server: Res<AssetServer>,
    mut current_music: ResMut<CurrentMusic>,
    mut commands: Commands,
) {
    for event in events.read() {
        if let LevelEvent::Spawned(spawned_iid) = event {
            // Get the LDtk project handle from the entity
            for project_handle in project_handles.iter() {
                // Get the actual project data from assets
                if let Some(project) = ldtk_assets.get(&project_handle.handle) {
                    // Find the level by IID using the official API
                    if let Some(level) = project.get_raw_level_by_iid(&spawned_iid.to_string()) {
                        // Check if we already have music for this level
                        if current_music.level_name == level.identifier {
                            break; // Skip if already playing music for this level
                        }

                        if let Some(music_path) = get_level_music(level) {
                            debug!("Spawning music: {} for level: {}", music_path, level.identifier);

                            // Despawn previous music if it exists
                            if current_music.entity != Entity::PLACEHOLDER {
                                commands.entity(current_music.entity).despawn();
                            }

                            // Load and spawn new music
                            let music_handle: Handle<AudioSource> = asset_server.load(format!(".copyright_music/{}", music_path));
                            let music_entity = commands.spawn((
                                AudioPlayer::new(music_handle),
                                PlaybackSettings::LOOP,
                            )).id();

                            // Update current music resource
                            current_music.entity = music_entity;
                            current_music.level_name = level.identifier.clone();
                        }

                        // Break after finding the level (only spawn music once per event)
                        break;
                    }
                }
            }
        }
    }
}
