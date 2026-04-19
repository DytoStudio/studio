//! The system for loading and unloading image tiles as needed.

use std::collections::HashSet;

use bevy::{
    camera::Camera,
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::ChildOf,
        query::With,
        system::{Commands, Query},
    },
    math::{DVec2, DVec3, Vec2},
    transform::components::{GlobalTransform, Transform},
};
use big_space::prelude::{CellCoord, Grids};

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
fn lod_for_height(height: f64) -> mercator::LevelOfDetail {
    mercator::LevelOfDetail::new(
        (28.0 - height.log2().ceil()).clamp(0.0, 255.0) as u8,
    )
}

/// Get the tiles within a bounding box.
fn tiles_in_bounding_box(
    result: &mut Vec<mercator::TileCoordinate>,
    top_left_world: DVec2,
    top_right_world: DVec2,
    bottom_left_world: DVec2,
    bottom_right_world: DVec2,
    lod: mercator::LevelOfDetail,
) {
    let tile_size = lod.ground_resolution(0.0) * mercator::TILE_SIZE_FLOAT;

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

    fn cross(a: DVec2, b: DVec2) -> f64 {
        a.x * b.y - a.y * b.x
    }

    let half_tiles = (1i64 << lod.value()) / 2;

    for tile in 0..total_tiles {
        let x = tile as isize % bounding_box_size_x + bounding_box_top_left_x;
        let y = tile as isize / bounding_box_size_x + bounding_box_top_left_y;
        let positions = [
            DVec2::new(x as f64, y as f64),
            DVec2::new(x as f64 + 1.0, y as f64),
            DVec2::new(x as f64 + 1.0, y as f64 + 1.0),
            DVec2::new(x as f64, y as f64 + 1.0),
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
                if corner.x >= x as f64
                    && corner.x < x as f64 + 1.0
                    && corner.y >= y as f64
                    && corner.y < y as f64 + 1.0
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
    camera_position: &DVec3,
    global_transform: &GlobalTransform,
) -> Option<Box<[mercator::TileCoordinate]>> {
    let viewport_size = camera.logical_viewport_size()?;

    fn raycast_to_ground(
        camera: &Camera,
        camera_position: &DVec3,
        global_transform: &GlobalTransform,
        position: Vec2,
    ) -> Option<DVec2> {
        let raycast =
            camera.viewport_to_world(global_transform, position).ok()?;

        let direction = DVec3::new(
            raycast.direction.x as f64,
            raycast.direction.y as f64,
            raycast.direction.z as f64,
        );
        if direction.y.abs() < f64::EPSILON {
            return None;
        }
        let ground_multiplier = -camera_position.y / direction.y;
        if ground_multiplier < 0.0 {
            return None;
        }

        let raycast_world = DVec2::new(camera_position.x, camera_position.z)
            + DVec2::new(
                direction.x * ground_multiplier,
                direction.z * ground_multiplier,
            );

        Some(raycast_world)
    }
    let top_left_world = raycast_to_ground(
        camera,
        camera_position,
        global_transform,
        Vec2::new(0.0, 0.0),
    )?;
    let top_right_world = raycast_to_ground(
        camera,
        camera_position,
        global_transform,
        Vec2::new(viewport_size.x, 0.0),
    )?;
    let bottom_left_world = raycast_to_ground(
        camera,
        camera_position,
        global_transform,
        Vec2::new(0.0, viewport_size.y),
    )?;
    let bottom_right_world = raycast_to_ground(
        camera,
        camera_position,
        global_transform,
        Vec2::new(viewport_size.x, viewport_size.y),
    )?;

    let lod = lod_for_height(camera_position.y).value();
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
        (&ChildOf, &Transform, &Camera, &CellCoord, &GlobalTransform),
        With<scene_camera::ImageTileLoadingCamera>,
    >,
    mut tiles: Query<(&ChildOf, Entity, &ImageTile), With<ManageableImageTile>>,
    grids: Grids,
) {
    // Get the required tiles for all cameras.
    let mut required_tiles: HashSet<(Entity, mercator::TileCoordinate)> =
        HashSet::new();
    for (child_of, transform, camera, cell_coord, global_transform) in
        cameras.iter_mut()
    {
        let grid = grids.get(child_of.parent());
        let world_coord = grid.cell_to_float(cell_coord);
        let camera_position = world_coord
            + DVec3::new(
                transform.translation.x as f64,
                transform.translation.y as f64,
                transform.translation.z as f64,
            );

        let Some(visible) =
            visible_tiles(camera, &camera_position, global_transform)
        else {
            continue;
        };
        required_tiles
            .extend(visible.into_iter().map(|tile| (child_of.parent(), tile)));
    }

    // Despawn unrequired tiles.
    for (child_of, entity, tile) in tiles.iter_mut() {
        if required_tiles.contains(&(child_of.parent(), tile.coordinate)) {
            // The tile is required, so we keep it and remove it from the
            // required tiles set.
            required_tiles.remove(&(child_of.parent(), tile.coordinate));
        } else {
            // The tile is not required, so we despawn it.
            commands.entity(entity).despawn();
        }
    }

    for (entity, tile) in required_tiles.into_iter() {
        // Spawn required tiles that are not already spawned.
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                ImageTileState::Uninitialized,
                ImageTile { coordinate: tile },
                ManageableImageTile,
                CellCoord::ZERO,
            ));
        });
    }
}
