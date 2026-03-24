//! The module for image tiles.

use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{IntoScheduleConfigs, SystemSet},
};

pub mod image_tile_loader;
pub mod manageable_image_tile;
pub mod mercator;

pub use image_tile_loader::{ImageTile, ImageTileState};
pub use manageable_image_tile::ManageableImageTile;

/// System sets for image tiles.
///
/// todo: consider splitting MoveCamera into seperate systems (such as
///       ZoomCamera and PanCamera).
#[derive(SystemSet, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ImageTileSystem {
    /// Manage the image tiles.
    ManageImageTiles,
    /// Load the image tiles.
    LoadImageTiles,
}

/// The plugin for managing image tiles.
pub struct ImageTilePlugin;

impl Plugin for ImageTilePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                ImageTileSystem::ManageImageTiles,
                ImageTileSystem::LoadImageTiles,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            manageable_image_tile::manageable_image_tile_update
                .in_set(ImageTileSystem::ManageImageTiles),
        );
        app.add_systems(
            Update,
            image_tile_loader::load_image_tile_update
                .in_set(ImageTileSystem::LoadImageTiles),
        );
    }
}
