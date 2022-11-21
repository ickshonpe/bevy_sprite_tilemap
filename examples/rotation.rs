use bevy::prelude::*;
use bevy_sprite_tilemap::prelude::*;

#[derive(Component)]
struct Center;

fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = 16.0 * Vec2::ONE;
    let atlas_grid = Tilemap::from_fn(4, 3, |x, y| TextureAtlasTile::new(y * 4 + x));
    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let grid_geometry = TilemapGeometry {
        tile_size,
        ..Default::default()
    };

    commands.spawn(TextureAtlasTilemapBundle {
        tilemap: atlas_grid.clone(),
        geometry: grid_geometry,
        texture_atlas: texture_atlas_handle.clone(),
        ..Default::default()
    });
}

fn rotate(time: Res<Time>, mut query: Query<&mut Transform, With<Tilemap<TextureAtlasTile>>>) {
    query.for_each_mut(|mut transform| {
        transform.rotate_z(0.1 * time.delta_seconds());
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grid)
        .add_system(rotate)
        .run();
}
