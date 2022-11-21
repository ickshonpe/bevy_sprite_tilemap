use bevy::prelude::*;
use bevy_sprite_tilemap::prelude::*;

fn spawn_grids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = 16.0 * Vec2::ONE;
    let atlas_grid = Tilemap::from_fn(9, 7, |x, y| TextureAtlasTile::new((y * 4 + x) % 16));
    let mut atlas_grid_dark = atlas_grid.clone();
    for sprite in &mut atlas_grid_dark {
        sprite.color = Color::DARK_GRAY;
    }
    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let grid_geometry = TilemapGeometry {
        tile_size,
        ..Default::default()
    };

    commands.spawn(TextureAtlasTilemapBundle {
        tilemap: atlas_grid_dark.clone(),
        geometry: grid_geometry.clone(),
        view: TilemapView::All,
        texture_atlas: texture_atlas_handle.clone(),

        ..Default::default()
    });
    commands.spawn(TextureAtlasTilemapBundle {
        tilemap: atlas_grid.clone(),
        geometry: grid_geometry,
        view: TilemapView::Section {
            x: 2,
            y: 3,
            width: 150,
            height: 120,
        },
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_translation(10. * Vec3::Z),
        ..Default::default()
    });
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
