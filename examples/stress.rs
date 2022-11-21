use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_sprite_tilemap::prelude::*;
use bevy_sprite_tilemap::tile::TextureAtlasTile;

fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tilemap = Tilemap::from_fn(200, 200, |x, y| TextureAtlasTile::new((x + y) % 16));
    let tile_size = 16.0 * Vec2::ONE;

    let grid_geometry = TilemapGeometry {
        tile_size,
        anchor: Anchor::Center,
        ..Default::default()
    };

    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_atlas_image, vec2(16., 16.), 4, 4, None, None);

    commands.spawn(TextureAtlasTilemapBundle {
        tilemap,
        geometry: grid_geometry,
        texture_atlas: texture_atlases.add(texture_atlas),
        transform: Transform::from_translation(100.0 * Vec3::Z),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1000.,
                height: 1000.,
                present_mode: bevy::window::PresentMode::Immediate,
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grid)
        .run();
}
