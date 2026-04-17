//! A movable camera system that supports zooming and panning.

use bevy::{
    camera::Camera,
    ecs::{component::Component, query::With, system::Query},
    input::{
        ButtonState,
        mouse::{MouseButton, MouseButtonInput, MouseWheel},
    },
    math::{Vec2, Vec3, primitives::InfinitePlane3d},
    prelude::MessageReader,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

/// The sensitivity of the camera zooming.
const ZOOM_SENSITIVITY: f32 = 0.01;
/// The lerp factor for camera zooming animation.
const ZOOM_LERP_FACTOR: f32 = 0.1;
/// The lerp factor for camera panning animation.
const PAN_LERP_FACTOR: f32 = 0.1;
/// The minimal difference between the current camera height and the target
/// height to consider the animation complete.
const MINIMAL_DIFFERENCE: f32 = 0.1;
/// The minimal height the camera can zoom to.
const MIN_CAMERA_HEIGHT: f32 = 1.0;
/// The maximal height the camera can zoom to.
const MAX_CAMERA_HEIGHT: f32 = 1000000000.0;

/// A component that allows the camera to be moved with user input.
#[derive(Component, Debug)]
pub struct MovableCamera {
    /// The world position of the point the user clicked on to pan the camera.
    ///
    /// If the user is not currently panning the camera, this will be `None`.
    pan_scene_position: Option<Vec2>,
    /// Whether the camera is currently animating towards the target height.
    ///
    /// If false, the camera instantly moves to the target height.
    animating_height: bool,
    /// The target height the camera should move to.
    target_height: f32,
    /// Whether the camera is currently animating towards the target position.
    ///
    /// If false, the camera instantly moves to the target position.
    animating_position: bool,
    /// The target position the camera should move to.
    target_position: Vec2,
}

impl Default for MovableCamera {
    fn default() -> Self {
        Self {
            pan_scene_position: None,
            animating_height: false,
            target_height: 5.0,

            animating_position: false,
            target_position: Vec2::ZERO,
        }
    }
}

/// Handles user input to move the camera during `Update`.
pub fn move_camera_update(
    mut query: Query<(
        &mut Transform,
        &mut MovableCamera,
        &Camera,
        &GlobalTransform,
    )>,
    mut scroll: MessageReader<MouseWheel>,
    mut button: MessageReader<MouseButtonInput>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    // Get the scroll wheel delta.
    let mut delta = Vec2::ZERO;
    for event in scroll.read() {
        delta += Vec2::new(event.x, event.y);
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

    for (mut transform, mut camera, camera_component, global_transform) in
        query.iter_mut()
    {
        // Start panning the camera.
        if let Some(pan_world_pos) = camera.pan_scene_position {
            if let Some(cursor) = cursor_position
                && let Ok(raycast) =
                    camera_component.viewport_to_world(global_transform, cursor)
                && let Some(intersection) = raycast
                    .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
            {
                let current_world_pos = Vec2::new(
                    raycast.origin.x + raycast.direction.x * intersection,
                    raycast.origin.z + raycast.direction.z * intersection,
                );
                let delta = pan_world_pos - current_world_pos;
                camera.target_position += delta;
                camera.animating_position = false;
            }

            // Detect letting go of the mouse button to stop panning.
            if mouse_button_pressed == Some(false) {
                camera.pan_scene_position = None;
            }
        } else if mouse_button_pressed == Some(true)
            && let Some(cursor) = cursor_position
            && let Ok(raycast) =
                camera_component.viewport_to_world(global_transform, cursor)
            && let Some(intersection) = raycast
                .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
        {
            camera.pan_scene_position = Some(Vec2::new(
                raycast.origin.x + raycast.direction.x * intersection,
                raycast.origin.z + raycast.direction.z * intersection,
            ));
        }

        // Zoom the camera.
        if let Some(cursor) = cursor_position
            && let Ok(raycast) =
                camera_component.viewport_to_world(global_transform, cursor)
            && let Some(intersection) = raycast
                .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))
        {
            let world_pos = raycast.origin + raycast.direction * intersection;
            let height = camera.target_height;
            let speed = height * ZOOM_SENSITIVITY;
            let new_height = (height - delta.y * speed)
                .clamp(MIN_CAMERA_HEIGHT, MAX_CAMERA_HEIGHT);
            let zoom_ratio = new_height / height;

            camera.animating_height = false;
            camera.target_height = new_height;
            camera.target_position.x = world_pos.x
                + (camera.target_position.x - world_pos.x) * zoom_ratio;
            camera.target_position.y = world_pos.z
                + (camera.target_position.y - world_pos.z) * zoom_ratio;
        }

        // Animate the camera height.
        let mut height = camera.target_height;
        if camera.animating_height {
            let difference = transform.translation.y - camera.target_height;
            if difference.abs() < MINIMAL_DIFFERENCE {
                height = camera.target_height;
                camera.animating_height = false;
            } else {
                height =
                    transform.translation.y - difference * ZOOM_LERP_FACTOR;
            }
        }

        // Animate the camera position.
        let mut position = camera.target_position;
        if camera.animating_position {
            let current_position =
                Vec2::new(transform.translation.x, transform.translation.z);
            let difference = current_position - camera.target_position;
            if difference.length().abs() < MINIMAL_DIFFERENCE {
                position = camera.target_position;
                camera.animating_position = false;
            } else {
                position = current_position - difference * PAN_LERP_FACTOR;
            }
        }

        transform.translation = Vec3::new(position.x, height, position.y);
    }
}
