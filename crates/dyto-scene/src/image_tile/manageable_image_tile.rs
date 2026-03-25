//! The system for loading and unloading image tiles as needed.

use std::collections::HashSet;

use bevy::{
    camera::Camera,
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    math::{Vec2, Vec3, primitives::InfinitePlane3d},
    transform::components::GlobalTransform,
};

use crate::{
    image_tile::{ImageTile, ImageTileState, mercator},
    scene_camera,
};

/// Additional levels of detail to load.
const ADDITIONAL_LOD_LEVELS: u8 = 10;

/// The component for a manageable image tile.
#[derive(Component, Debug)]
pub struct ManageableImageTile;

/// Get the level of detail for a given camera height.
fn lod_for_height(height: f32) -> mercator::LevelOfDetail {
    mercator::LevelOfDetail::new(
        (28.0 - height.log2().ceil()).clamp(0.0, 255.0) as u8,
    )
}

/// Get the tiles within a bounding box.
fn tiles_in_bounding_box(
    result: &mut Vec<mercator::TileCoordinate>,
    top_left_world: Vec2,
    top_right_world: Vec2,
    bottom_left_world: Vec2,
    bottom_right_world: Vec2,
    lod: mercator::LevelOfDetail,
) {
    let tile_size =
        (lod.ground_resolution(0.0) * mercator::TILE_SIZE_FLOAT) as f32;

    // +- 1 is added to make sure the bounding box covers all the tiles that
    // are partially covered by the camera view.
    let bounding_box_top_left_x = ((top_left_world.x / tile_size).floor()
        as isize)
        .min((bottom_left_world.x / tile_size).floor() as isize)
        .min((top_right_world.x / tile_size).floor() as isize)
        .min((bottom_right_world.x / tile_size).floor() as isize)
        - 1;
    let bounding_box_top_left_y = ((top_left_world.y / tile_size).floor()
        as isize)
        .min((bottom_left_world.y / tile_size).floor() as isize)
        .min((top_right_world.y / tile_size).floor() as isize)
        .min((bottom_right_world.y / tile_size).floor() as isize)
        - 1;
    let bounding_box_bottom_right_x = ((top_left_world.x / tile_size).ceil()
        as isize)
        .max((bottom_left_world.x / tile_size).ceil() as isize)
        .max((top_right_world.x / tile_size).ceil() as isize)
        .max((bottom_right_world.x / tile_size).ceil() as isize)
        + 1;
    let bounding_box_bottom_right_y = ((top_left_world.y / tile_size).ceil()
        as isize)
        .max((bottom_left_world.y / tile_size).ceil() as isize)
        .max((top_right_world.y / tile_size).ceil() as isize)
        .max((bottom_right_world.y / tile_size).ceil() as isize)
        + 1;
    let bounding_box_size_x =
        bounding_box_bottom_right_x - bounding_box_top_left_x + 1;
    let bounding_box_size_y =
        bounding_box_bottom_right_y - bounding_box_top_left_y + 1;

    let a = top_left_world / tile_size;
    let b = top_right_world / tile_size;
    let c = bottom_right_world / tile_size;
    let d = bottom_left_world / tile_size;

    let ab_edge = b - a;
    let bc_edge = c - b;
    let cd_edge = d - c;
    let da_edge = a - d;

    let total_tiles = (bounding_box_size_x * bounding_box_size_y) as usize;
    result.reserve(total_tiles);

    fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    let half_tiles = (1i64 << lod.value()) / 2;

    for tile in 0..total_tiles {
        let x = tile as isize % bounding_box_size_x + bounding_box_top_left_x;
        let y = tile as isize / bounding_box_size_x + bounding_box_top_left_y;
        let positions = [
            Vec2::new(x as f32, y as f32),
            Vec2::new(x as f32 + 1.0, y as f32),
            Vec2::new(x as f32 + 1.0, y as f32 + 1.0),
            Vec2::new(x as f32, y as f32 + 1.0),
        ];

        let mut inside = false;
        for position in positions {
            let ab = cross(ab_edge, position - a);
            let bc = cross(bc_edge, position - b);
            let cd = cross(cd_edge, position - c);
            let da = cross(da_edge, position - d);

            if (ab >= 0.0 && bc >= 0.0 && cd >= 0.0 && da >= 0.0)
                || (ab <= 0.0 && bc <= 0.0 && cd <= 0.0 && da <= 0.0)
            {
                inside = true;
                break;
            }
        }

        // If the tile is not inside the camera view, check if any of its
        // corners are
        if !inside {
            for corner in [a, b, c, d] {
                if corner.x >= x as f32
                    && corner.x < x as f32 + 1.0
                    && corner.y >= y as f32
                    && corner.y < y as f32 + 1.0
                {
                    inside = true;
                    break;
                }
            }
        }

        let actual_x = x + half_tiles as isize;
        let actual_y = y + half_tiles as isize;
        if inside && actual_y >= 0 && actual_y < (1 << lod.value()) as isize {
            result.push(mercator::TileCoordinate::new(
                actual_x.rem_euclid(1 << lod.value()) as u32,
                actual_y as u32,
                lod,
            ));
        }
    }
}

/// Get the visible tiles for a given camera.
fn visible_tiles(
    camera: &Camera,
    global_transform: &GlobalTransform,
) -> Option<Box<[mercator::TileCoordinate]>> {
    let viewport_size = camera.logical_viewport_size()?;
    fn raycast_to_ground(
        camera: &Camera,
        global_transform: &GlobalTransform,
        position: Vec2,
    ) -> Option<Vec2> {
        let raycast =
            camera.viewport_to_world(global_transform, position).ok()?;
        let intersection = raycast
            .intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))?;
        let world_pos = raycast.origin + raycast.direction * intersection;
        Some(Vec2::new(world_pos.x, world_pos.z))
    }
    let top_left_world =
        raycast_to_ground(camera, global_transform, Vec2::new(0.0, 0.0))?;
    let top_right_world = raycast_to_ground(
        camera,
        global_transform,
        Vec2::new(viewport_size.x, 0.0),
    )?;
    let bottom_left_world = raycast_to_ground(
        camera,
        global_transform,
        Vec2::new(0.0, viewport_size.y),
    )?;
    let bottom_right_world = raycast_to_ground(
        camera,
        global_transform,
        Vec2::new(viewport_size.x, viewport_size.y),
    )?;

    let lod = lod_for_height(global_transform.translation().y).value();
    let mut result = Vec::new();

    for lod in
        (((lod as i8 - ADDITIONAL_LOD_LEVELS as i8).max(0) as u8)..=lod).rev()
    {
        tiles_in_bounding_box(
            &mut result,
            top_left_world,
            top_right_world,
            bottom_left_world,
            bottom_right_world,
            mercator::LevelOfDetail::new(lod),
        );
    }

    Some(result.into_boxed_slice())
}

/// Load and unload image tiles as needed on `Update` based on SceneCamera
/// position and zoom level.
pub fn manageable_image_tile_update(
    mut commands: Commands,
    mut cameras: Query<
        (&Camera, &GlobalTransform),
        With<scene_camera::ImageTileLoadingCamera>,
    >,
    mut tiles: Query<(Entity, &ImageTile), With<ManageableImageTile>>,
) {
    // Get the required tiles for all cameras.
    let mut required_tiles: HashSet<mercator::TileCoordinate> = HashSet::new();
    for (camera, global_transform) in cameras.iter_mut() {
        let Some(visible) = visible_tiles(camera, global_transform) else {
            continue;
        };
        required_tiles.extend(visible.into_iter());
    }

    // Despawn unrequired tiles.
    for (entity, tile) in tiles.iter_mut() {
        if required_tiles.contains(&tile.coordinate) {
            // The tile is required, so we keep it and remove it from the
            // required tiles set.
            required_tiles.remove(&tile.coordinate);
        } else {
            // The tile is not required, so we despawn it.
            commands.entity(entity).despawn();
        }
    }

    // Spawn required tiles that are not already spawned.
    for tile in required_tiles.into_iter() {
        commands.spawn((
            ImageTileState::Uninitialized,
            ImageTile { coordinate: tile },
            ManageableImageTile,
        ));
    }
}
