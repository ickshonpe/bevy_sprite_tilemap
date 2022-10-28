use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Clone, Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TilemapGeometry {
    /// size of each grid cell
    /// textures are not strectched or shrunk to fill cells
    pub tile_size: Vec2,
    /// if false draw rows in reverse order (index increases from right to left)
    pub reverse_rows: bool,
    /// if false draw columns in reverse order (index increases from top to bottom)
    pub reverse_columns: bool,
    /// how the grid is positioned relative to its transform
    pub anchor: Anchor,
}

impl Default for TilemapGeometry {
    fn default() -> Self {
        Self {
            tile_size: 16. * Vec2::ONE,
            anchor: Anchor::Center,
            reverse_rows: false,
            reverse_columns: false,
        }
    }
}

#[derive(Clone, Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum TilemapView {
    /// Draw the entire tilemap
    #[default]
    All,
    /// Draw a rectangular subsection of the tilemap
    Section {
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    },
}
