use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

pub fn update_position(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in &mut query {
        position.0 += velocity.0 * time.delta_seconds();
    }
}

pub fn sync_position(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut query {
        transform.translation = position.0.extend(0.);
    }
}
