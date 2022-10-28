use crate::extraction::ExtractableAtlasTilemap;
use crate::extraction::ExtractableTilemap;
use crate::prelude::*;
use crate::tile::SpriteTile;
use crate::tile::TextureAtlasTile;
use bevy::prelude::*;
use bevy::sprite::ExtractedSprite;

impl ExtractableAtlasTilemap for Tilemap<TextureAtlasTile> {
    #[inline]
    fn extract_tile(
        &self,
        entity: Entity,
        transform: GlobalTransform,
        texture_atlas: &TextureAtlas,
        index: usize,
    ) -> Option<ExtractedSprite> {
        let sprite = &self[index];
        ExtractedSprite {
            entity,
            transform,
            color: sprite.color,
            rect: Some(texture_atlas.textures[sprite.index]),
            custom_size: sprite.custom_size,
            image_handle_id: texture_atlas.texture.id,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            anchor: sprite.anchor.as_vec(),
        }
        .into()
    }
}

impl ExtractableAtlasTilemap for Tilemap<Option<TextureAtlasTile>> {
    #[inline]
    fn extract_tile(
        &self,
        entity: Entity,
        transform: GlobalTransform,
        texture_atlas: &TextureAtlas,
        index: usize,
    ) -> Option<ExtractedSprite> {
        self[index].as_ref().map(|sprite| ExtractedSprite {
            entity,
            transform,
            color: sprite.color,
            rect: Some(texture_atlas.textures[sprite.index]),
            custom_size: sprite.custom_size,
            image_handle_id: texture_atlas.texture.id,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            anchor: sprite.anchor.as_vec(),
        })
    }
}

impl ExtractableTilemap for Tilemap<SpriteTile> {
    #[inline]
    fn extract_tile(
        &self,
        entity: Entity,
        transform: GlobalTransform,
        index: usize,
    ) -> Option<ExtractedSprite> {
        let sprite = &self[index];
        ExtractedSprite {
            entity,
            transform,
            color: sprite.color,
            rect: None,
            custom_size: sprite.custom_size,
            image_handle_id: sprite.texture.id,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            anchor: sprite.anchor.as_vec(),
        }
        .into()
    }
}

impl ExtractableTilemap for Tilemap<Option<SpriteTile>> {
    #[inline]
    fn extract_tile(
        &self,
        entity: Entity,
        transform: GlobalTransform,
        index: usize,
    ) -> Option<ExtractedSprite> {
        self[index].as_ref().map(|sprite| ExtractedSprite {
            entity,
            transform,
            color: sprite.color,
            rect: None,
            custom_size: sprite.custom_size,
            image_handle_id: sprite.texture.id,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            anchor: sprite.anchor.as_vec(),
        })
    }
}
