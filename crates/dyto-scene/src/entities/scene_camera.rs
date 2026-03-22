use bevy::{
    camera::Camera,
    ecs::{component::Component, query::With, system::Query},
    input::mouse::MouseWheel,
    math::{Vec2, Vec3, primitives::InfinitePlane3d},
    prelude::{KeyModifierState, KeyModifiers, MessageReader},
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};

const ZOOM_SENSITIVITY: f32 = 0.01;
const PAN_SENSITIVITY: f32 = 0.001;
const ZOOM_LERP_FACTOR: f32 = 0.1;
const PAN_LERP_FACTOR: f32 = 0.1;
const MINIMAL_DIFFERENCE: f32 = 0.1;
const MIN_CAMERA_HEIGHT: f32 = 1.0;
const MAX_CAMERA_HEIGHT: f32 = 10000.0;

#[derive(Component)]
pub struct SceneCamera {
    has_control_key: bool,

    animating_height: bool,
    target_height: f32,

    animating_position: bool,
    target_position: Vec2,
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self {
            has_control_key: false,
            animating_height: false,
            target_height: 5.0,

            animating_position: false,
            target_position: Vec2::ZERO,
        }
    }
}

impl SceneCamera {
    fn update_camera_position(&mut self, transform: &mut Transform) {
        let mut height = self.target_height;
        if self.animating_height {
            let difference = transform.translation.y - self.target_height;
            if difference.abs() < MINIMAL_DIFFERENCE {
                height = self.target_height;
                self.animating_height = false;
            } else {
                height =
                    transform.translation.y - difference * ZOOM_LERP_FACTOR;
            }
        }

        let mut position = self.target_position;
        if self.animating_position {
            let current_position =
                Vec2::new(transform.translation.x, transform.translation.z);
            let difference = current_position - self.target_position;
            if difference.length().abs() < MINIMAL_DIFFERENCE {
                position = self.target_position;
                self.animating_position = false;
            } else {
                position = current_position - difference * PAN_LERP_FACTOR;
            }
        }

        transform.translation = Vec3::new(position.x, height, position.y);
    }

    pub fn update(
        mut query: Query<(
            &mut Transform,
            &mut SceneCamera,
            &Camera,
            &GlobalTransform,
        )>,
        mut scroll: MessageReader<MouseWheel>,
        mut modifiers: MessageReader<KeyModifiers>,
        window: Query<&Window, With<PrimaryWindow>>,
    ) {
        let mut delta = Vec2::ZERO;
        for event in scroll.read() {
            delta += Vec2::new(event.x, event.y);
        }
        let mut control_key_pressed: Option<bool> = None;
        for event in modifiers.read() {
            control_key_pressed =
                Some(event.state.contains(KeyModifierState::CONTROL));
        }

        let cursor_position = window
            .single()
            .ok()
            .and_then(|w| w.cursor_position())
            .unwrap_or(Vec2::ZERO);

        for (mut transform, mut camera, camera3d, global_transform) in
            query.iter_mut()
        {
            if let Some(pressed) = control_key_pressed {
                camera.has_control_key = pressed;
            }
            if delta.y != 0.0 && camera.has_control_key {
                let raycast = camera3d
                    .viewport_to_world(global_transform, cursor_position);
                if let Ok(raycast) = raycast {
                    let intersection = raycast
                        .intersect_plane(
                            Vec3::ZERO,
                            InfinitePlane3d::new(Vec3::Y),
                        )
                        .unwrap_or(0.0);

                    let world_pos =
                        raycast.origin + raycast.direction * intersection;
                    let old_height = transform.translation.y;
                    let speed = old_height * ZOOM_SENSITIVITY;
                    let new_height = (old_height - delta.y * speed)
                        .clamp(MIN_CAMERA_HEIGHT, MAX_CAMERA_HEIGHT);
                    let zoom_ratio = new_height / old_height;

                    camera.animating_height = false;
                    camera.target_height = (transform.translation.y
                        - delta.y * speed)
                        .clamp(MIN_CAMERA_HEIGHT, MAX_CAMERA_HEIGHT);
                    camera.target_position.x = world_pos.x
                        + (transform.translation.x - world_pos.x) * zoom_ratio;
                    camera.target_position.y = world_pos.z
                        + (transform.translation.z - world_pos.z) * zoom_ratio;
                }
            } else {
                // camera panning
                let height = transform.translation.y;
                let speed = height * PAN_SENSITIVITY;
                camera.animating_position = false;
                camera.target_position.x -= delta.x * speed;
                camera.target_position.y -= delta.y * speed;
            }

            camera.update_camera_position(&mut transform);
        }
    }
}
