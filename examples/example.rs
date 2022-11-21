use bevy::math::vec2;
use bevy::prelude::*;
use bevy_sprite_tilemap::prelude::*;

fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = 16.0 * Vec2::ONE;
    let atlas_grid = Tilemap::from_fn(3, 4, |x, y| TextureAtlasTile::new(x + y * 4));
    let grid_geometry = TilemapGeometry {
        tile_size: 2. * tile_size,
        ..Default::default()
    };

    let texture_atlas_image = asset_server.load("test_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_atlas_image, tile_size, 4, 4, None, None);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::WHITE,
            custom_size: Some(
                vec2(
                    atlas_grid.width() as f32 + 1.0,
                    atlas_grid.height() as f32 + 1.0,
                ) * grid_geometry.tile_size,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    let pos = -0.5
        * grid_geometry.tile_size
        * vec2(
            atlas_grid.width() as f32 - 1.0,
            atlas_grid.height() as f32 - 1.0,
        );
    for x in 0..atlas_grid.width() {
        for y in 0..atlas_grid.height() {
            let target = pos + vec2(x as f32, y as f32) * grid_geometry.tile_size;
            let color = if (x + y) % 2 == 0 {
                Color::DARK_GRAY
            } else {
                Color::GRAY
            };
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(grid_geometry.tile_size),
                    ..Default::default()
                },
                transform: Transform::from_translation(target.extend(10.0)),
                ..Default::default()
            });
        }
    }

    commands.spawn(TextureAtlasTilemapBundle {
        tilemap: atlas_grid,
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
                width: 300.0,
                height: 300.0,
                scale_factor_override: Some(2.0),
                ..Default::default()
            },
            ..Default::default()
        }))
        .add_plugin(SpriteTilemapPlugin)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_startup_system(spawn_grid)
        .run();
}
