use geometry::TilemapGeometry;

use crate::geometry;
use bevy::prelude::*;

pub fn pick_tile(
    world_point: Vec2,
    transform: &GlobalTransform,
    width: usize,
    mheight: usize,
    geometry: TilemapGeometry,
) {
    let grid_space_point = transform
        .affine()
        .transform_vector3(world_point.extend(0.0));

    // let grid_dimensions = vec2(width as f32, height as f32);
    // let grid_size = grid_dimensions * geometry.tile_size;
    // let grid_translation = transform.affine().transform_vector3(
    //     (0.5 * geometry.tile_size - (0.5 + geometry.anchor.as_vec()) * grid_size
    //         + vec2(
    //             if geometry.reverse_rows {
    //                 grid_size.x - geometry.tile_size.x
    //             } else {
    //                 0.0
    //             },
    //             if geometry.reverse_columns {
    //                 grid_size.y - geometry.tile_size.y
    //             } else {
    //                 0.0
    //             },
    //         ))
    //     .extend(0.),
    // );
}
