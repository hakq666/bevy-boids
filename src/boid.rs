use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::movement::{Position, Velocity};
use crate::spatial_grid::{Grid, SquareQuery};
use crate::{GameCamera, GameSettings};

#[derive(Component, Debug)]
pub struct Boid;

#[derive(Bundle)]
pub struct BoidBundle {
    pub boid: Boid,
    pub position: Position,
    pub velocity: Velocity,
    pub sprite: SpriteBundle,
}

pub fn spawn_boid(position: Vec2, velocity: Vec2, commands: &mut Commands) {
    commands.spawn(BoidBundle {
        boid: Boid,
        position: Position(position),
        velocity: Velocity(velocity),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50., 50.)),
                ..default()
            },
            ..default()
        }, // TODO: need to add boid sprite
    });
}

pub fn spawn_boid_on_mouseclick(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(cursor_position) = window.cursor_position() {
            let Some(world_position) =
                camera.viewport_to_world_2d(camera_transform, cursor_position)
            else {
                return;
            };

            spawn_boid(world_position, Vec2::ZERO, &mut commands);
        }
    }
}

fn flock(
    mut query: Query<(Entity, &Position, &mut Velocity), With<Boid>>,
    spatial_grid: Res<Grid>,
    game_settings: Res<GameSettings>,
) {
    for (entity, position, mut velocity) in &query {
        let neighbors_query = SquareQuery::new(position.0, game_settings.visual_range);

        let mut perceived_flock_velocity = Vec2::ZERO;
        let mut perceived_flock_position = Vec2::ZERO;

        for (neighbor_entity, neighbor_position) in spatial_grid.query(neighbors_query) {
            if entity == neighbor_entity {
                continue;
            }

            todo!()
        }
    }
}
