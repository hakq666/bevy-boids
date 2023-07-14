use bevy::prelude::*;

use crate::spatial_grid::Grid;

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

pub fn update_position(
    mut query: Query<(Entity, &mut Position, &Velocity)>,
    time: Res<Time>,
    mut spatial_grid: ResMut<Grid>,
) {
    for (entity, mut position, velocity) in &mut query {
        let new_position = position.0 + velocity.0 * time.delta_seconds();

        spatial_grid.update(entity, position.0, new_position);
        position.0 = new_position;
    }
}

pub fn sync_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation = position.0.extend(0.);
    }
}
