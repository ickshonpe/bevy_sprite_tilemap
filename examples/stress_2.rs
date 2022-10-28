use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_sprite_tilemap::prelude::*;
use bevy_sprite_tilemap::tile::SpriteTile;

fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    let paths = ["a.png", "b.png", "c.png", "d.png"];
    let tilemap = Tilemap::from_fn(200, 200, |x, _y| {
        SpriteTile::new(asset_server.load(paths[x % 4]))
    });
    let tile_size = 16.0 * Vec2::ONE;

    let geometry = TilemapGeometry {
        tile_size,
        anchor: Anchor::Center,
        ..Default::default()
    };

    commands.spawn_bundle(SpriteTilemapBundle {
        tilemap,
        geometry,
        transform: Transform::from_translation(100.0 * Vec3::Z),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1000.,
            height: 1000.,
            present_mode: bevy::window::PresentMode::Immediate,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grid)
        .run();
}
