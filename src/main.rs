mod boid;
mod movement;
mod spatial_grid;

use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::boid::spawn_boid_on_mouseclick;
use crate::movement::{sync_position, update_position};

pub const WINDOW_TITLE: &str = "Bevy Boids";
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Component, Debug)]
pub struct GameCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: WINDOW_TITLE.into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (update_position, sync_position).chain(),
                spawn_boid_on_mouseclick,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}
