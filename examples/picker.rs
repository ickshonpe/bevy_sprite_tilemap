use std::cell;
use std::f32::consts::PI;

use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::sprite::Anchor;
use bevy_sprite_tilemap::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = Vec2::splat(16.);
    let tiles = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(tiles, tile_size, 4, 4, None, None);
    commands.spawn(TextureAtlasTilemapBundle {
        tilemap: Tilemap::from_fn(4, 4, |x, y| TextureAtlasTile::new(x + y * 4)),
        texture_atlas: texture_atlases.add(texture_atlas),
        geometry: TilemapGeometry {
            tile_size,
            anchor: Anchor::BottomLeft,
            reverse_columns: true,
            ..Default::default()
        },
        transform: Transform {
            translation: -100. * Vec3::Y,
            rotation: Quat::from_rotation_z(PI / 4.),
            scale: (Vec2::splat(100.) / tile_size).extend(1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn picking(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut tilemap_query: Query<(
        &mut Tilemap<TextureAtlasTile>,
        &TilemapGeometry,
        &GlobalTransform,
    )>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };
    if let Some(point) = windows
        .get_primary()
        .and_then(|window| window.cursor_position())
    {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (point / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_point: Vec2 = world_pos.truncate();
        let (mut tilemap, geometry, transform) = tilemap_query.single_mut();

        let grid_dimensions = vec2(tilemap.width() as f32, tilemap.height() as f32);
        let grid_size = grid_dimensions * geometry.tile_size;

        // let grid_translation =
        //     0.5 * geometry.tile_size - (0.5 + geometry.anchor.as_vec()) * grid_size
        //     + vec2(
        //         if geometry.reverse_rows {
        //             grid_size.x - geometry.tile_size.x
        //         } else {
        //             0.0
        //         },
        //         if geometry.reverse_columns {
        //             grid_size.y - geometry.tile_size.y
        //         } else {
        //             0.0
        //         },
        //     );

        let grid_translation = (0.5 + geometry.anchor.as_vec()) * grid_size;
        //let transformed_grid_translation = transform.affine().transform_vector3(grid_translation.extend(0.0));

        let translated_point = world_point - transform.translation().truncate();
        let grid_space_point = transform
            .affine()
            .inverse()
            .transform_vector3(translated_point.extend(0.))
            + grid_translation.extend(0.);
        let cell_point = grid_space_point.truncate() / geometry.tile_size;
        let index = if (0.0..grid_dimensions.x).contains(&cell_point.x)
            && (0.0..grid_dimensions.y).contains(&cell_point.y)
        {
            let x = if geometry.reverse_rows {
                tilemap.width() - cell_point.x as usize - 1
            } else {
                cell_point.x as usize
            };
            let y = if geometry.reverse_columns {
                tilemap.width() - cell_point.y as usize - 1
            } else {
                cell_point.y as usize
            };
            Some([x, y])
        } else {
            None
        };
        //println!("grid_anchor_translation {} -> transformed_grid_anchor_translation {}", grid_translation, transformed_grid_translation);
        println!("cursor point {point} -> world point {world_point} -> grid_transform_space {grid_space_point} -> cell point {cell_point} -> index {:?}", index);
        if let Some(index) = index {
            for (x, y, tile) in tilemap.indexed_iter_mut() {
                if [x, y] == index {
                    tile.color = Color::WHITE;
                } else {
                    tile.color = Color::DARK_GRAY;
                }
            }
        } else {
            for (x, y, tile) in tilemap.indexed_iter_mut() {
                tile.color = Color::DARK_GRAY;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_grid)
        .add_system(picking)
        .run();
}
