//! A movable camera system that supports zooming and panning.

use bevy::{
    camera::Camera,
    ecs::{
        component::Component, hierarchy::ChildOf, query::With, system::Query,
    },
    input::{
        ButtonState,
        mouse::{MouseButton, MouseButtonInput, MouseWheel},
    },
    math::{DVec2, DVec3},
    prelude::MessageReader,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};
use big_space::prelude::{CellCoord, Grids};

/// The sensitivity of the camera zooming.
const ZOOM_SENSITIVITY: f64 = 0.01;
/// The lerp factor for camera zooming animation.
const ZOOM_LERP_FACTOR: f64 = 0.1;
/// The lerp factor for camera panning animation.
const PAN_LERP_FACTOR: f64 = 0.1;
/// The minimal difference between the current camera height and the target
/// height to consider the animation complete.
const MINIMAL_DIFFERENCE: f64 = 0.1;
/// The minimal height the camera can zoom to.
const MIN_CAMERA_HEIGHT: f64 = 1.0;
/// The maximal height the camera can zoom to.
const MAX_CAMERA_HEIGHT: f64 = 1000000000.0;

/// A component that allows the camera to be moved with user input.
#[derive(Component, Debug)]
pub struct MovableCamera {
    /// The world position of the point the user clicked on to pan the camera.
    ///
    /// If the user is not currently panning the camera, this will be `None`.
    pan_scene_position: Option<DVec2>,
    /// Whether the camera is currently animating towards the target height.
    ///
    /// If false, the camera instantly moves to the target height.
    animating_height: bool,
    /// The target height the camera should move to.
    target_height: f64,
    /// Whether the camera is currently animating towards the target position.
    ///
    /// If false, the camera instantly moves to the target position.
    animating_position: bool,
    /// The target position the camera should move to.
    target_position: DVec2,
}

impl Default for MovableCamera {
    fn default() -> Self {
        Self {
            pan_scene_position: None,
            animating_height: false,
            target_height: 5.0,

            animating_position: false,
            target_position: DVec2::ZERO,
        }
    }
}

/// Handles user input to move the camera during `Update`.
pub fn move_camera_update(
    mut query: Query<(
        &ChildOf,
        &mut CellCoord,
        &mut Transform,
        &mut MovableCamera,
        &Camera,
        &GlobalTransform,
    )>,
    mut scroll: MessageReader<MouseWheel>,
    mut button: MessageReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
    grids: Grids,
) {
    // Get the scroll wheel delta.
    let mut delta = DVec2::ZERO;
    for event in scroll.read() {
        delta += DVec2::new(event.x as f64, event.y as f64);
    }

    // Check for updates in the control key state.
    let mut mouse_button_pressed: Option<bool> = None;
    for event in button.read() {
        if event.button != MouseButton::Left {
            continue;
        }
        mouse_button_pressed = Some(event.state == ButtonState::Pressed);
    }

    let cursor_position =
        window.single().ok().and_then(|w| w.cursor_position());

    for (
        child_of,
        mut cell_coord,
        mut transform,
        mut camera,
        camera_component,
        global_transform,
    ) in query.iter_mut()
    {
        // Get the current world position.
        let grid = grids.get(child_of.parent());
        let world_coord = grid.cell_to_float(&cell_coord);
        let current_world = world_coord
            + DVec3::new(
                transform.translation.x as f64,
                transform.translation.y as f64,
                transform.translation.z as f64,
            );
        let current_world2 = DVec2::new(current_world.x, current_world.z);

        // Do a mouse raycast if needed.
        let mut cached_raycast: Option<Option<DVec2>> = None;
        let mut get_raycast = || -> Option<DVec2> {
            if let Some(raycast) = cached_raycast {
                return raycast;
            }

            let Some(cursor) = cursor_position else {
                cached_raycast = Some(None);
                return None;
            };
            let Ok(raycast) =
                camera_component.viewport_to_world(global_transform, cursor)
            else {
                cached_raycast = Some(None);
                return None;
            };
            let direction = DVec3::new(
                raycast.direction.x as f64,
                raycast.direction.y as f64,
                raycast.direction.z as f64,
            );
            if direction.y.abs() < f64::EPSILON {
                cached_raycast = Some(None);
                return None;
            }
            let ground_multiplier = -current_world.y / direction.y;
            if ground_multiplier < 0.0 {
                cached_raycast = Some(None);
                return None;
            }
            let raycast_world = current_world2
                + DVec2::new(
                    direction.x * ground_multiplier,
                    direction.z * ground_multiplier,
                );
            cached_raycast = Some(Some(raycast_world));

            Some(raycast_world)
        };

        // Start panning the camera.
        if let Some(pan_world) = camera.pan_scene_position {
            if let Some(mouse_world) = get_raycast() {
                let delta = pan_world - mouse_world;
                camera.target_position += delta;
                camera.animating_position = false;
            }

            // Detect letting go of the mouse button to stop panning.
            if mouse_button_pressed == Some(false) {
                camera.pan_scene_position = None;
            }
        } else if mouse_button_pressed == Some(true)
            && let Some(mouse_world) = get_raycast()
        {
            camera.pan_scene_position = Some(mouse_world);
        }

        // Zoom the camera.
        if let Some(mouse_world) = get_raycast() {
            let height = camera.target_height;
            let speed = height * ZOOM_SENSITIVITY;
            let new_height = (height - delta.y * speed)
                .clamp(MIN_CAMERA_HEIGHT, MAX_CAMERA_HEIGHT);
            let zoom_ratio = new_height / height;

            camera.animating_height = false;
            camera.target_height = new_height;
            camera.target_position.x = mouse_world.x
                + (camera.target_position.x - mouse_world.x) * zoom_ratio;
            camera.target_position.y = mouse_world.y
                + (camera.target_position.y - mouse_world.y) * zoom_ratio;
        }

        // Animate the camera height.
        let mut height = camera.target_height;
        if camera.animating_height {
            let difference = current_world.y - camera.target_height;
            if difference.abs() < MINIMAL_DIFFERENCE {
                height = camera.target_height;
                camera.animating_height = false;
            } else {
                height = current_world.y - difference * ZOOM_LERP_FACTOR;
            }
        }

        // Animate the camera position.
        let mut position = camera.target_position;
        if camera.animating_position {
            let difference = current_world2 - camera.target_position;
            if difference.length().abs() < MINIMAL_DIFFERENCE {
                position = camera.target_position;
                camera.animating_position = false;
            } else {
                position = current_world2 - difference * PAN_LERP_FACTOR;
            }
        }

        let (new_coord, new_translation) = grid
            .translation_to_grid(DVec3::new(position.x, height, position.y));
        *cell_coord = new_coord;
        transform.translation = new_translation;
    }
}
