//! Camera systems for the scene.

use bevy::{
    app::{App, Plugin, Update},
    ecs::{component::Component, schedule::SystemSet},
};

pub mod movable_camera;

pub use movable_camera::MovableCamera;

/// Any camera that should load and unload image tiles.
#[derive(Component, Debug)]
pub struct ImageTileLoadingCamera;

/// System sets for the scene camera.
///
/// todo: consider splitting MoveCamera into separate systems (such as
///       ZoomCamera and PanCamera).
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SceneCameraSystem {
    /// Move the camera.
    MoveCamera,
}

/// The plugin for the scene camera.
pub struct SceneCameraPlugin;

impl Plugin for SceneCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movable_camera::move_camera_update);
    }
}
