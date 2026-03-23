//! Camera systems for the scene.

use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::SystemSet,
};

pub mod movable_camera;

pub use movable_camera::MovableCamera;

/// System sets for the scene camera.
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SceneCameraSystem {
    /// Moves the camera.
    MoveCamera,
    /// Load and unload tiles.
    ManageTiles,
}

/// The plugin for the scene camera.
pub struct SceneCameraPlugin;

impl Plugin for SceneCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movable_camera::move_camera_update);
    }
}
