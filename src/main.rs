use bevy::{prelude::*, window::{PresentMode}};

pub const WINDOW_TITLE: &str = "Bevy Boids";
pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

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
    .run();
}
