//! The scene entry point.

use bevy::{
    app::PluginGroup,
    camera::{Camera3d, ClearColor},
    color::Color,
    light::DirectionalLight,
    math::primitives::Cuboid,
    mesh::Mesh3d,
    pbr::MeshMaterial3d,
    prelude::{
        App, Assets, Commands, DefaultPlugins, EulerRot, Mesh, Quat, ResMut,
        StandardMaterial, Startup, Transform, Vec3, Window, WindowPlugin,
    },
};

use crate::scene_camera;

/// The main scene struct that holds the Bevy app.
pub struct Scene {
    app: App,
}

impl Scene {
    /// Create a new scene with the default plugins and a primary window.
    #[cfg(feature = "web")]
    pub fn new(canvas_selector: &str) -> Self {
        let mut app = App::new();

        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(canvas_selector.to_string()),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }));
        app.add_plugins(scene_camera::SceneCameraPlugin);
        app.add_systems(Startup, Self::setup);

        app.insert_resource(ClearColor(Color::BLACK));

        Self { app }
    }

    /// Run the scene.
    #[cfg(feature = "web")]
    pub fn run(&mut self) {
        self.app.run();
    }

    /// Setup the scene with a camera, light, and a cube.
    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::NEG_Z),
            scene_camera::MovableCamera::default(),
        ));

        commands.spawn((
            DirectionalLight::default(),
            Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -0.5,
                -0.5,
                0.0,
            )),
        ));

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.8))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
    }
}
