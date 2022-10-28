use crate::indexing::IndexableGrid;
use crate::tile::Tileable;
use bevy::prelude::*;

#[derive(Clone, Component, Debug)]
pub struct Tilemap<T>
where
    T: Tileable,
{
    /// tile data
    tiles: Vec<T>,
    /// width of grid in cells
    width: usize,
    /// height of grid in cells
    height: usize,
}

impl<T> IndexableGrid for Tilemap<T>
where
    T: Tileable,
{
    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    #[inline]
    fn height(&self) -> usize {
        self.height
    }
}

impl<T> Default for Tilemap<T>
where
    T: Tileable,
{
    fn default() -> Self {
        Self {
            tiles: vec![T::default()],
            width: 1,
            height: 1,
        }
    }
}

impl<T> Tilemap<T>
where
    T: Tileable + Clone,
{
    pub fn from_elem(width: usize, height: usize, initial_sprite: T) -> Self {
        Self {
            tiles: vec![initial_sprite; width * height],
            width,
            height,
        }
    }
}

impl<T> Tilemap<T>
where
    T: Tileable,
{
    pub fn from_fn(width: usize, height: usize, mut f: impl FnMut(usize, usize) -> T) -> Self {
        let sprites = (0..width * height)
            .map(|i| f(i % width, i / width))
            .collect();
        Self {
            tiles: sprites,
            width,
            height,
        }
    }

    pub fn from_default(width: usize, height: usize) -> Self {
        Self::from_elem(width, height, T::default())
    }
}

impl<U> Tilemap<Option<U>>
where
    U: Tileable,
{
    #[inline]
    pub fn remove(&mut self, x: usize, y: usize) -> Option<U> {
        self[[x, y]].take()
    }
}

impl<T> std::ops::Index<[usize; 2]> for Tilemap<T>
where
    T: Tileable,
{
    type Output = T;

    #[inline]
    fn index(&self, [x, y]: [usize; 2]) -> &Self::Output {
        &self.tiles[self.index_grid(x, y)]
    }
}

impl<T> std::ops::IndexMut<[usize; 2]> for Tilemap<T>
where
    T: Tileable,
{
    #[inline]
    fn index_mut(&mut self, [x, y]: [usize; 2]) -> &mut Self::Output {
        let index = self.index_grid(x, y);
        &mut self.tiles[index]
    }
}

impl<T> std::ops::Index<usize> for Tilemap<T>
where
    T: Tileable,
{
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.tiles[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Tilemap<T>
where
    T: Tileable,
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tiles[index]
    }
}

impl<T> IntoIterator for Tilemap<T>
where
    T: Tileable,
{
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Tilemap<T>
where
    T: Tileable,
{
    type Item = &'a T;

    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Tilemap<T>
where
    T: Tileable,
{
    type Item = &'a mut T;

    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.iter_mut()
    }
}

impl<T> Tilemap<T>
where
    T: Tileable,
{
    pub fn indexed_iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        let mut x = 0;
        let mut y = 0;
        self.into_iter().map(move |tile| {
            let out = (x, y, tile);
            x += 1;
            if x == self.width() {
                x = 0;
                y += 1;
            }
            out
        })
    }

    pub fn indexed_iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        let width = self.width;
        let mut x = 0;
        let mut y = 0;
        self.into_iter().map(move |tile| {
            let out = (x, y, tile);
            x += 1;
            if x == width {
                x = 0;
                y += 1;
            }
            out
        })
    }
}
