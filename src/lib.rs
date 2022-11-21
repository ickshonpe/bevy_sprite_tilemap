pub mod bundles;
pub mod extractable_tilemaps;
pub mod extraction;
pub mod geometry;
pub mod indexing;
pub mod tile;
pub mod tilemap;
pub mod util;

use crate::geometry::*;

use bevy::prelude::*;

pub mod prelude {
    pub use crate::bundles::*;
    pub use crate::geometry::TilemapGeometry;
    pub use crate::geometry::TilemapView;
    pub use crate::indexing::*;
    pub use crate::tile::SpriteTile;
    pub use crate::tile::TextureAtlasTile;
    pub use crate::tile::Tileable;
    pub use crate::tilemap::*;
    pub use crate::SpriteTilemapPlugin;
}

pub struct SpriteTilemapPlugin;

impl Plugin for SpriteTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TilemapGeometry>()
            .register_type::<TilemapView>()
            .add_plugin(extraction::TilemapExtractionPlugin);
    }
}
