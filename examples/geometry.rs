use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_sprite_tilemap::prelude::*;

fn spawn_grids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = 16.0 * Vec2::ONE;
    let atlas_grid = Tilemap::from_fn(4, 4, |x, y| TextureAtlasTile::new(y * 4 + x));
    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for (d, reverse_rows, reverse_columns, anchor) in [
        (vec2(1., 1.), false, false, Anchor::BottomLeft),
        (vec2(-1., 1.), true, false, Anchor::BottomRight),
        (vec2(-1., -1.), true, true, Anchor::TopRight),
        (vec2(1., -1.), false, true, Anchor::TopLeft),
    ] {
        let grid_geometry = TilemapGeometry {
            tile_size,
            reverse_rows,
            reverse_columns,
            anchor,
        };
        commands.spawn(TextureAtlasTilemapBundle {
            tilemap: atlas_grid.clone(),
            geometry: grid_geometry,
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation((0.5 * d * tile_size).extend(0.)),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grids)
        .run();
}
