use bevy::math::vec2;
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
    let tile_size = 16.0 * Vec2::ONE;
    let mut atlas_grid = Tilemap::from_fn(4, 4, |x, y| TextureAtlasTile::new(y * 4 + x));
    atlas_grid[15].anchor = Anchor::BottomLeft;
    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpatialBundle::default())
        .insert(Center)
        .with_children(|commands| {
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
                    tilemap: atlas_grid.clone(),
                    geometry: grid_geometry,
                    texture_atlas: texture_atlas_handle.clone(),
                    view: TilemapView::Section {
                        x: 1,
                        y: 1,
                        width: 3,
                        height: 3,
                    },
                    transform: Transform::from_translation((0.5 * d * tile_size).extend(0.)),
                    ..Default::default()
                });
            }
        });
}

fn transformation(time: Res<Time>, mut query: Query<&mut Transform, With<Center>>) {
    let p = (0.5 * time.seconds_since_startup() as f32).sin();
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
            commands.spawn_bundle(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grids)
        .add_system(transformation)
        .run();
}
