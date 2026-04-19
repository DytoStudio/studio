//! Image tile components.

use std::{collections::HashMap, sync::Arc};

use bevy::{
    asset::{Assets, RenderAssetUsages},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        message::MessageReader,
        system::{Commands, Query, Res, ResMut},
    },
    image::Image,
    math::{DVec3, primitives::Plane3d},
    mesh::{Mesh, Mesh3d, Meshable},
    pbr::{MeshMaterial3d, StandardMaterial},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    transform::components::Transform,
};
use big_space::prelude::{CellCoord, Grids};

use crate::{
    image_tile::mercator,
    message_bridge::{self, SceneToEmbedderMessages, messages},
};

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

/// Load or request for image tiles during the `Update` stage.
#[allow(clippy::too_many_arguments)]
pub fn load_image_tile_update(
    mut commands: Commands,
    mut tiles: Query<(
        &ChildOf,
        Entity,
        &mut CellCoord,
        &ImageTile,
        &mut ImageTileState,
    )>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut loaded_tiles: MessageReader<messages::ImageTileLoaded>,
    message_bridge: Res<message_bridge::MessageBridge>,
    grids: Grids,
) {
    let mut image_data: HashMap<mercator::TileCoordinate, Arc<[u8]>> =
        HashMap::new();
    for message in loaded_tiles.read() {
        image_data.insert(message.0, message.1.clone());
    }

    let mut tiles_to_request = Vec::new();
    for (child_of, entity, mut cell_coord, tile, mut state) in tiles.iter_mut()
    {
        match *state {
            ImageTileState::Loaded => (),
            ImageTileState::Loading => {
                let Some(data) = image_data.get(&tile.coordinate) else {
                    continue;
                };
                let image = Image::new(
                    Extent3d {
                        width: mercator::TILE_SIZE,
                        height: mercator::TILE_SIZE,
                        depth_or_array_layers: 1,
                    },
                    TextureDimension::D2,
                    data.to_vec(),
                    TextureFormat::Rgba8UnormSrgb,
                    RenderAssetUsages::RENDER_WORLD,
                );

                let size =
                    tile.coordinate.level_of_detail().ground_resolution(0.0)
                        * mercator::TILE_SIZE_FLOAT;

                let world = tile.coordinate.as_world_position();

                let target_position = DVec3::new(world.0, 0.0, -world.1);
                let (coord, translation) = grids
                    .get(child_of.parent())
                    .translation_to_grid(target_position);

                *cell_coord = coord;

                commands.entity(entity).insert((
                    Mesh3d(
                        meshes.add(
                            Plane3d::default()
                                .mesh()
                                .size(size as f32, size as f32),
                        ),
                    ),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(images.add(image)),
                        unlit: true,
                        ..Default::default()
                    })),
                    Transform::from_translation(translation),
                ));

                *state = ImageTileState::Loaded;
            }
            ImageTileState::Uninitialized => {
                tiles_to_request.push(tile.coordinate);
                *state = ImageTileState::Loading;
            }
        }
    }

    if !tiles_to_request.is_empty() {
        (message_bridge.send_fn)(SceneToEmbedderMessages::RequestImageTile(
            tiles_to_request.into_boxed_slice(),
        ));
    }
}
