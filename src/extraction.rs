use crate::geometry::*;
use crate::prelude::IndexableGrid;
use crate::prelude::Tilemap;
use crate::tile::SpriteTile;
use crate::tile::TextureAtlasTile;
use bevy::math::vec2;
use bevy::math::Vec3A;
use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::VecHelper;

pub trait ExtractableAtlasTilemap: Component + IndexableGrid {
    fn extract_tile(
        &self,
        entity: Entity,
        transform: GlobalTransform,
        texture_atlas: &TextureAtlas,
        index: usize,
    ) -> Option<ExtractedSprite>;
}

pub trait ExtractableTilemap: Component + IndexableGrid {
    fn extract_tile(
        &self,
        entity: Entity,
        transform: GlobalTransform,
        index: usize,
    ) -> Option<ExtractedSprite>;
}

fn iter_grid_coords(
    grid_width: usize,
    grid_height: usize,
    view: &TilemapView,
    geometry: &TilemapGeometry,
    mut transform: GlobalTransform,
) -> impl Iterator<Item = (usize, GlobalTransform)> {
    let [view_x, view_y, view_width, view_height] = view.clip([grid_width, grid_height]);
    let grid_dimensions = vec2(grid_width as f32, grid_height as f32);
    let grid_size = grid_dimensions * geometry.tile_size;
    let grid_translation = transform.affine().transform_vector3(
        (0.5 * geometry.tile_size - (0.5 + geometry.anchor.as_vec()) * grid_size
            + vec2(
                if geometry.reverse_rows {
                    grid_size.x - geometry.tile_size.x
                } else {
                    0.0
                },
                if geometry.reverse_columns {
                    grid_size.y - geometry.tile_size.y
                } else {
                    0.0
                },
            ))
        .extend(0.),
    );
    let next_row_skip = grid_width - view_width;
    let right = transform
        .affine()
        .transform_vector3(geometry.tile_size.x * Vec3::X);
    let up = transform
        .affine()
        .transform_vector3(geometry.tile_size.y * Vec3::Y);
    let step_row: Vec3A = if geometry.reverse_rows { -1.0 } else { 1.0 } * Vec3A::from(right);
    let step_column: Vec3A = if geometry.reverse_columns { -1.0 } else { 1.0 } * Vec3A::from(up);
    let next_row_step: Vec3A = -step_row * view_width as f32 + step_column;
    let view_translation =
        Vec3A::from(grid_translation) + view_x as f32 * step_row + view_y as f32 * step_column;
    *transform.translation_mut() += view_translation;
    let mut cell_index = view_y * grid_width + view_x;
    let mut x = 0;
    (0..view_width * view_height).map(move |_| {
        let out = (cell_index, transform);
        *transform.translation_mut() += step_row;
        cell_index += 1;
        x += 1;
        if x == view_width {
            x = 0;
            cell_index += next_row_skip;
            *transform.translation_mut() += next_row_step;
        }
        out
    })
}

#[allow(clippy::type_complexity)]
pub fn extract_atlas_tilemap<T>(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    tilemap_query: Extract<
        Query<(
            Entity,
            &T,
            &TilemapGeometry,
            &TilemapView,
            &Handle<TextureAtlas>,
            &GlobalTransform,
            &ComputedVisibility,
        )>,
    >,
) where
    T: ExtractableAtlasTilemap,
{
    for (
        entity,
        tilemap,
        tilemap_geometry,
        tilemap_view,
        texture_atlas_handle,
        global_transform,
        visibility,
    ) in tilemap_query.iter()
    {
        if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
            if !visibility.is_visible() {
                return;
            }
            iter_grid_coords(
                tilemap.width(),
                tilemap.height(),
                tilemap_view,
                tilemap_geometry,
                *global_transform,
            )
            .for_each(|(index, transform)| {
                if let Some(extracted_sprite) =
                    tilemap.extract_tile(entity, transform, texture_atlas, index)
                {
                    extracted_sprites.sprites.alloc().init(extracted_sprite);
                }
            });
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn extract_tilemap<T>(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    tilemap_query: Extract<
        Query<(
            Entity,
            &T,
            &TilemapGeometry,
            &TilemapView,
            &GlobalTransform,
            &ComputedVisibility,
        )>,
    >,
) where
    T: ExtractableTilemap,
{
    for (entity, tilemap, tilemap_geometry, tilemap_view, global_transform, visibility) in
        tilemap_query.iter()
    {
        if !visibility.is_visible() {
            return;
        }
        iter_grid_coords(
            tilemap.width(),
            tilemap.height(),
            tilemap_view,
            tilemap_geometry,
            *global_transform,
        )
        .for_each(|(index, transform)| {
            if let Some(extracted_sprite) = tilemap.extract_tile(entity, transform, index) {
                extracted_sprites.sprites.alloc().init(extracted_sprite);
            }
        });
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TilemapRenderSystem {
    ExtractTiles,
}

pub(crate) struct TilemapExtractionPlugin;

impl Plugin for TilemapExtractionPlugin {
    fn build(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .add_system_to_stage(
                    RenderStage::Extract,
                    extract_atlas_tilemap::<Tilemap<TextureAtlasTile>>
                        .label(TilemapRenderSystem::ExtractTiles)
                        .after(SpriteSystem::ExtractSprites),
                )
                .add_system_to_stage(
                    RenderStage::Extract,
                    extract_atlas_tilemap::<Tilemap<Option<TextureAtlasTile>>>
                        .label(TilemapRenderSystem::ExtractTiles)
                        .after(SpriteSystem::ExtractSprites),
                )
                .add_system_to_stage(
                    RenderStage::Extract,
                    extract_tilemap::<Tilemap<SpriteTile>>
                        .label(TilemapRenderSystem::ExtractTiles)
                        .after(SpriteSystem::ExtractSprites),
                )
                .add_system_to_stage(
                    RenderStage::Extract,
                    extract_tilemap::<Tilemap<Option<SpriteTile>>>
                        .label(TilemapRenderSystem::ExtractTiles)
                        .after(SpriteSystem::ExtractSprites),
                );
        }
    }
}
