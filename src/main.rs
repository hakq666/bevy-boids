mod boid;
mod movement;
mod spatial_grid;

use bevy::prelude::*;
use bevy::window::PresentMode;

use crate::boid::spawn_boid_on_mouseclick;
use crate::movement::{sync_position, update_position};
use crate::spatial_grid::{update_grid_dimensions_on_window_resize, Grid};

pub const WINDOW_TITLE: &str = "Bevy Boids";
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

#[derive(Resource, Debug)]
pub struct GameSettings {
    pub boids_count: usize,
    pub separation_factor: f32,
    pub alignment_factor: f32,
    pub cohesion_factor: f32,
    pub visual_range: f32,
    pub separation_min_distance: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            boids_count: 100,
            separation_factor: 0.7,
            alignment_factor: 0.1,
            cohesion_factor: 0.05,
            visual_range: 100.,
            separation_min_distance: 20.,
        }
    }
}

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
        .init_resource::<Grid>()
        .init_resource::<GameSettings>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (update_position, sync_position).chain(),
                spawn_boid_on_mouseclick,
                update_grid_dimensions_on_window_resize,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), GameCamera));
}
