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
    let mut atlas_grid_dark = atlas_grid.clone();
    for sprite in &mut atlas_grid_dark {
        sprite.color = Color::DARK_GRAY;
    }
    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4);
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
        commands.spawn_bundle(TextureAtlasTilemapBundle {
            tilemap: atlas_grid_dark.clone(),
            geometry: grid_geometry.clone(),
            view: TilemapView::All,
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation((0.5 * d * tile_size).extend(0.)),
            ..Default::default()
        });
        commands.spawn_bundle(TextureAtlasTilemapBundle {
            tilemap: atlas_grid.clone(),
            geometry: grid_geometry,
            view: TilemapView::Section {
                x: 1,
                y: 1,
                width: 3,
                height: 3,
            },
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation((0.5 * d * tile_size).extend(100.)),
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grids)
        .run();
}
