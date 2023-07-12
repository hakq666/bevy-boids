use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::movement::{Position, Velocity};

#[derive(Component)]
pub struct Boid;

#[derive(Bundle)]
pub struct BoidBundle {
    boid: Boid,
    position: Position,
    velocity: Velocity,
    sprite: SpriteBundle,
}

pub fn spawn_boid(position: Vec2, velocity: Vec2, commands: &mut Commands) {
    commands.spawn(BoidBundle {
        boid: Boid,
        position: Position(position),
        velocity: Velocity(velocity),
        sprite: SpriteBundle { ..default() }, // TODO: need to add boid sprite
    });
}

pub fn spawn_boid_on_mouseclick(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let window = window_query.single();

        if let Some(position) = window.cursor_position() {
            spawn_boid(position, Vec2::ZERO, &mut commands);
        }
    }
}
