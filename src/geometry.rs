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

#[inline]
fn clip_axis(space: usize, min: usize, length: usize) -> usize {
    length.min(space.saturating_sub(min))
}

impl TilemapView {
    #[inline]
    pub fn clip(&self, map_size: [usize; 2]) -> [usize; 4] {
        match *self {
            TilemapView::All => [0, 0, map_size[0], map_size[1]],
            TilemapView::Section {
                x,
                y,
                width,
                height,
            } => [
                x,
                y,
                clip_axis(map_size[0], x, width),
                clip_axis(map_size[1], y, height),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clip_view() {
        let s = [15, 20];
        let clip = |views: TilemapView| {
            let [_, _, w, h] = views.clip(s);
            [w, h]
        };

        let view = TilemapView::Section {
            x: 0,
            y: 0,
            width: s[0],
            height: s[1],
        };
        assert_eq!(clip(view), s);

        let view = TilemapView::Section {
            x: 3,
            y: 4,
            width: 2,
            height: 5,
        };
        assert_eq!(clip(view), [2, 5]);

        let view = TilemapView::Section {
            x: s[0],
            y: s[1],
            width: 10,
            height: 10,
        };
        assert_eq!(clip(view), [0, 0]);

        let view = TilemapView::Section {
            x: s[0] - 1,
            y: s[1] - 1,
            width: 10,
            height: 10,
        };
        assert_eq!(clip(view), [1, 1]);

        let view = TilemapView::Section {
            x: 100,
            y: 0,
            width: 9,
            height: 11,
        };
        assert_eq!(clip(view), [0, 11]);

        let view = TilemapView::Section {
            x: 0,
            y: 37,
            width: 11,
            height: 10,
        };
        assert_eq!(clip(view), [11, 0]);

        let view = TilemapView::Section {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        assert_eq!(clip(view), s);

        let view = TilemapView::Section {
            x: 0,
            y: 10,
            width: 15,
            height: 15,
        };
        assert_eq!(clip(view), [15, 10]);
    }
}
