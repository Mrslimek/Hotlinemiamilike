use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{AttackCooldown, Enemy, Player},
    constants::{ATTACK_RANGE, PLAYER_SPEED},
    resources::GameState,
};

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Single<&mut Transform, With<Player>>,
) {
    let mut transform = query.into_inner();

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

    let movement = direction * PLAYER_SPEED * time.delta().as_secs_f32();
    transform.translation.x += movement.x;
    transform.translation.y += movement.y;
}

pub fn player_attack(
    mouse: Res<ButtonInput<MouseButton>>,
    q_windows: Single<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    game_state: ResMut<GameState>,
    mut commands: Commands,
    player_query: Single<(Entity, &Transform, &mut AttackCooldown), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy)>,
    asset_server: Res<AssetServer>,
) {
    if game_state.game_over {
        return;
    }

    let (_player_entity, player_transform, mut cooldown) = player_query.into_inner();

    cooldown.0.tick(time.delta());

    if mouse.just_pressed(MouseButton::Left) && cooldown.0.is_finished() {
        cooldown.0.reset();

        // Play attack sound
        commands.spawn(AudioPlayer::new(asset_server.load("player_attack.ogg")));

        let window = q_windows.into_inner();

        if let Some(cursor_pos) = window.cursor_position() {
            let world_mouse_pos = Vec2::new(
                cursor_pos.x - window.width() / 2.0,
                -(cursor_pos.y - window.height() / 2.0),
            );

            let attack_direction =
                (world_mouse_pos - player_transform.translation.truncate()).normalize_or_zero();

            for (_enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
                let to_enemy = enemy_transform.translation.truncate()
                    - player_transform.translation.truncate();
                let distance = to_enemy.length();

                if distance <= ATTACK_RANGE {
                    let dot_product = to_enemy.normalize_or_zero().dot(attack_direction);

                    if dot_product > 0.0 {
                        enemy.health -= 1;
                    }
                }
            }
        }
    }
}
