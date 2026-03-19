use bevy::{
    app::PluginGroup,
    camera::Camera3d,
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

pub struct DytoScene {
    app: App,
}

impl DytoScene {
    // web feature only
    #[cfg(feature = "web")]
    pub fn new(canvas_selector: &str) -> Self {
        let app = App::new();

        let mut instance = Self { app };

        instance.app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some(canvas_selector.to_string()),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }));
        instance.app.add_systems(Startup, Self::setup);

        instance
    }

    #[cfg(feature = "web")]
    pub fn run(mut self) {
        self.app.run();
    }

    pub fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        commands.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
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
