use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Goal;

const TOTAL_LEVELS: usize = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LevelFlow {
            index: 0,
            total: TOTAL_LEVELS,
        })
        .insert_resource(GameState {
            reached_goal: false,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, apply_ldtk_entity_blueprints)
        .add_systems(Update, player_movement)
        .add_systems(Update, goal_interaction)
        .add_systems(Update, advance_level_on_goal)
        .add_systems(Update, camera_follow_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle {
            handle: asset_server.load("levels/HotlineMiamiLikeWorld.ldtk"),
        },
        ..Default::default()
    });
}

#[derive(Resource)]
struct GameState {
    reached_goal: bool,
}

#[derive(Resource)]
struct LevelFlow {
    index: usize,
    total: usize,
}

fn apply_ldtk_entity_blueprints(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &EntityInstance, &mut Transform), Added<EntityInstance>>,
) {
    for (entity, instance, mut transform) in query.iter_mut() {
        match instance.identifier.as_str() {
            "Player" => {
                transform.translation.z = 10.0;
                commands
                    .entity(entity)
                    .insert((Sprite::from_image(asset_server.load("player.png")), Player));
            }
            "Enemy" => {
                transform.translation.z = 10.0;
                commands
                    .entity(entity)
                    .insert((Sprite::from_image(asset_server.load("enemy.png")), Enemy));
            }
            "Wall" => {
                transform.translation.z = 5.0;
                commands.entity(entity).insert((
                    Sprite {
                        color: Color::srgb(0.65, 0.55, 0.40),
                        custom_size: Some(Vec2::splat(16.0)),
                        ..default()
                    },
                    Wall,
                ));
            }
            "Goal" => {
                transform.translation.z = 9.0;
                commands.entity(entity).insert((
                    Sprite {
                        color: Color::srgb(0.2, 0.9, 0.35),
                        custom_size: Some(Vec2::splat(16.0)),
                        ..default()
                    },
                    Goal,
                ));
            }
            _ => {}
        }
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction != Vec2::ZERO {
        direction = direction.normalize();
    }

    let speed = 200.0;
    let movement = direction * speed * time.delta().as_secs_f32();
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}

fn goal_interaction(
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    goal_query: Query<&Transform, With<Goal>>,
) {
    if game_state.reached_goal {
        return;
    }

    let Ok(player) = player_query.single() else {
        return;
    };

    for goal in goal_query.iter() {
        let distance = player
            .translation
            .truncate()
            .distance(goal.translation.truncate());
        if distance <= 20.0 {
            game_state.reached_goal = true;
            info!("Goal reached!");
            break;
        }
    }
}

fn advance_level_on_goal(
    mut game_state: ResMut<GameState>,
    mut level_flow: ResMut<LevelFlow>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if !game_state.reached_goal {
        return;
    }

    game_state.reached_goal = false;

    level_flow.index += 1;
    if level_flow.index >= level_flow.total {
        level_flow.index = 0;
        info!("All levels completed. Looping back to level_0.");
    } else {
        info!("Loading level_{}...", level_flow.index);
    }

    *level_selection = LevelSelection::index(level_flow.index);
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut camera) = camera_query.single_mut() else {
        return;
    };

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
