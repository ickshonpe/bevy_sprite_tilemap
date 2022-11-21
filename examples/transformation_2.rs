use bevy::math::vec3;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_sprite_tilemap::prelude::*;

#[derive(Component)]
struct Center;

fn spawn_grids(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let width = 7;
    let height = 6;
    let tile_size = 16.0 * Vec2::ONE;
    let mut atlas_grid = Tilemap::from_fn(width, height, |x, y| {
        TextureAtlasTile::new((y * width + x) % 16)
    });
    atlas_grid[width * height - 1].anchor = Anchor::BottomLeft;
    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Center)
        .with_children(|builder| {
            for (color, reverse_rows, reverse_columns, anchor) in [
                (Color::WHITE, false, false, Anchor::BottomLeft),
                (Color::RED, true, false, Anchor::BottomRight),
                (Color::GREEN, true, true, Anchor::TopRight),
                (Color::BLUE, false, true, Anchor::TopLeft),
            ] {
                let grid_geometry = TilemapGeometry {
                    tile_size,
                    reverse_rows,
                    reverse_columns,
                    anchor,
                };
                for sprite in &mut atlas_grid {
                    sprite.color = color;
                }
                builder.spawn_bundle(TextureAtlasTilemapBundle {
                    tilemap: atlas_grid.clone(),
                    geometry: grid_geometry,
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(vec3(50.0, -10.0, 0.0)),
                    ..Default::default()
                });
            }
        });
}

fn transformation(time: Res<Time>, mut query: Query<&mut Transform, With<Center>>) {
    let p = (0.5 * time.elapsed_seconds() as f32).sin();
    query.for_each_mut(|mut transform| {
        transform.rotation = Quat::from_rotation_z(std::f32::consts::PI * p);
        transform.scale = (2.5 + p) * Vec2::ONE.extend(0.);
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
        .add_system(transformation)
        .run();
}
