use bevy::prelude::*;
use bevy::sprite::Anchor;

pub trait Tileable: 'static + Send + Sync + Default + Clone {}

impl<T> Tileable for Option<T> where T: Tileable {}
impl Tileable for TextureAtlasTile {}
impl Tileable for SpriteTile {}

#[derive(Component, Debug, Default, Clone, Reflect)]
pub struct TextureAtlasTile {
    /// index of the image in the texture atlas
    pub index: usize,
    /// The tile's color tint
    pub color: Color,
    /// Flip the sprite along the `X` axis
    pub flip_x: bool,
    /// Flip the sprite along the `Y` axis
    pub flip_y: bool,
    /// The size of the tile in the grid
    pub custom_size: Option<Vec2>,
    /// [`Anchor`] point of the sprite in the world
    pub anchor: Anchor,
}

impl TextureAtlasTile {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Default, Clone, Reflect)]
pub struct SpriteTile {
    /// Asset handle for the tile's texture
    pub texture: Handle<Image>,
    /// The tile's color tint
    pub color: Color,
    /// Flip the sprite along the `X` axis
    pub flip_x: bool,
    /// Flip the sprite along the `Y` axis
    pub flip_y: bool,
    /// The size of the tile in the grid
    pub custom_size: Option<Vec2>,
    /// [`Anchor`] point of the sprite in the world
    pub anchor: Anchor,
}

impl SpriteTile {
    pub fn new(texture: Handle<Image>) -> Self {
        Self {
            texture,
            ..Default::default()
        }
    }
}
