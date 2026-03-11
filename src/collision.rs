use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::components::{Collider, Enemy, Player, Wall};

/// Resolves AABB collision between a moving entity and a static wall.
/// Pushes the moving entity out on the axis with the smallest overlap.
fn push_out_of_wall(mover_pos: &mut Vec3, mover_half: Vec2, wall_pos: Vec3, wall_half: Vec2) {
    let mover_aabb = Aabb2d::new(mover_pos.truncate(), mover_half);
    let wall_aabb = Aabb2d::new(wall_pos.truncate(), wall_half);

    if !mover_aabb.intersects(&wall_aabb) {
        return;
    }

    let diff = mover_pos.truncate() - wall_pos.truncate();
    let overlap_x = (mover_half.x + wall_half.x) - diff.x.abs();
    let overlap_y = (mover_half.y + wall_half.y) - diff.y.abs();

    if overlap_x < overlap_y {
        mover_pos.x += overlap_x * diff.x.signum();
    } else {
        mover_pos.y += overlap_y * diff.y.signum();
    }
}

/// Resolves collisions between the player and all wall entities.
pub fn player_wall_collision(
    mut player_query: Query<(&mut Transform, &Collider), With<Player>>,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Player>)>,
) {
    let Ok((mut player_transform, player_collider)) = player_query.single_mut() else {
        return;
    };

    for (wall_transform, wall_collider) in wall_query.iter() {
        push_out_of_wall(
            &mut player_transform.translation,
            player_collider.half_size,
            wall_transform.translation,
            wall_collider.half_size,
        );
    }
}

/// Resolves collisions between each enemy and all wall entities.
pub fn enemy_wall_collision(
    mut enemy_query: Query<(&mut Transform, &Collider), (With<Enemy>, Without<Wall>)>,
    wall_query: Query<(&Transform, &Collider), (With<Wall>, Without<Enemy>)>,
) {
    for (mut enemy_transform, enemy_collider) in enemy_query.iter_mut() {
        for (wall_transform, wall_collider) in wall_query.iter() {
            push_out_of_wall(
                &mut enemy_transform.translation,
                enemy_collider.half_size,
                wall_transform.translation,
                wall_collider.half_size,
            );
        }
    }
}
