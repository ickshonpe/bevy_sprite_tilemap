use bevy::prelude::*;
use bevy_sprite_tilemap::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_grid(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteTilemapBundle {
        tilemap: Tilemap::from_elem(3, 2, SpriteTile::new(asset_server.load("d.png"))),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_grid)
        .run();
}
