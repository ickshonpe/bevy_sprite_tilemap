use crate::tile::SpriteTile;
use crate::tile::TextureAtlasTile;
use crate::tilemap::Tilemap;
use crate::*;

#[derive(Bundle, Default)]
pub struct SpriteTilemapBundle {
    pub tilemap: Tilemap<SpriteTile>,
    pub geometry: TilemapGeometry,
    pub view: TilemapView,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Default)]
pub struct SparseSpriteTilemapBundle {
    pub tilemap: Tilemap<Option<SpriteTile>>,
    pub geometry: TilemapGeometry,
    pub view: TilemapView,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Default)]
pub struct TextureAtlasTilemapBundle {
    pub tilemap: Tilemap<TextureAtlasTile>,
    pub geometry: TilemapGeometry,
    pub view: TilemapView,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

#[derive(Bundle, Default)]
pub struct SparseAtlasTilemapBundle {
    pub tilemap: Tilemap<Option<TextureAtlasTile>>,
    pub geometry: TilemapGeometry,
    pub view: TilemapView,
    pub texture_atlas: Handle<TextureAtlas>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}
