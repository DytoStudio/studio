//! Image tile components.

use bevy::{
    // camera::Camera,
    color::Color,
    ecs::{component::Component, system::Query},
    gizmos::gizmos::Gizmos,
    math::{Isometry3d, Quat, Vec2, Vec3},
};

use crate::image_tile::mercator;

/// The loading state of the image tile.
#[derive(Component, Debug)]
pub enum ImageTileState {
    /// The image tile is currently loaded and visible.
    Loaded,
    /// The image tile is currently loading.
    Loading,
    /// The image tile is uninitialized and not visible.
    Uninitialized,
}

/// A drawable image tile.
#[derive(Component, Debug)]
pub struct ImageTile {
    /// The coordinate of the image tile.
    pub coordinate: mercator::TileCoordinate,
}

/// Draw gizmos for image tiles durring the `Update` stage.
pub fn load_image_tile_update(
    tiles: Query<(&ImageTile, &ImageTileState)>,
    mut gizmos: Gizmos,
) {
    for (tile, _state) in tiles.iter() {
        let color = Color::WHITE;
        let size =
            tile.coordinate.level_of_detail().ground_resolution(0.0) * 256.0;
        let world = tile.coordinate.as_world_position();
        let x = world.0 as f32;
        let z = -world.1 as f32;

        gizmos.rect(
            Isometry3d::new(
                Vec3::new(x, 0.0, z),
                Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ),
            Vec2::splat(size as f32),
            color,
        );
    }
}
