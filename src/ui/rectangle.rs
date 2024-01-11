/*
 * Copyright (c)  Sebastian Riga 2023.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software
 * and associated
 * documentation files (the “Software”), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies
 * or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR
 * PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::cmp::{max, min};
use std::fmt::{Display, Formatter};

use crate::core::dimension_2d::Dimension2d;
use crate::core::position_2d::Position2d;
use crate::ui::tile::MapTile;
use crate::ui::tile_map::TileMap;

/// Presents a two dimensional rectangular box in the cartesian coordinate system.
///
/// Can be rendered on a [TileMap] to construct message boxes, dialogs, menus, rooms and corridors:
///
/// ```text
///
/// ##############
/// #...........##########################
/// #.....................................
/// #...........##########################
/// #...........#
/// ##############
/// ```
///
/// # Properties
///
/// * `left`: The left most coordinate of the rectangle on the horizontal `x-axis`.
/// * `bottom`: The lowest coordinate of the rectangle on the vertical `y-axis`.
/// * `right`: The right most coordinate of the rectangle on the horizontal `x-axis`.
/// * `top`: The highest coordinate of the rectangle on the vertical `y-axis`.
///
/// # Examples
///
/// ```
/// let rect = Rectangle::new([34, 12], [40, 15]);
///
/// assert_eq!(34, rect.left);
/// assert_eq!(12, rect.bottom);
/// assert_eq!(74, rect.right);
/// assert_eq!(27, rect.top);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    /// The left most coordinate of the rectangle on the horizontal `x-axis`.
    pub left: i32,
    /// The lowest coordinate of the rectangle on the vertical `y-axis`.
    pub bottom: i32,
    /// The right most coordinate of the rectangle on the horizontal `x-axis`.
    pub right: i32,
    /// The highest coordinate of the rectangle on the vertical `y-axis`.
    pub top: i32,
}

impl Rectangle {
    /// Creates a new rectangle at the given `origin` with the passed `dimension`.
    ///
    /// # Arguments
    ///
    /// * `origin`: The origin [Position2d] of the [Rectangle].
    /// * `dimension`: The [Dimension2d] of the [Rectangle].
    ///
    /// returns: [Rectangle]
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    /// # See also
    ///
    /// * [Position2d]
    /// * [Dimension2d]
    ///
    pub fn new(origin: impl Position2d, dimension: impl Dimension2d) -> Self {
        Self {
            left: origin.x_coordinate(),
            bottom: origin.y_coordinate(),
            right: origin.x_coordinate() + dimension.width(),
            top: origin.y_coordinate() + dimension.height(),
        }
    }

    /// Checks if the passed [Rectangle] and the calling one overlap at any point.
    ///
    /// # Arguments
    ///
    /// * `other`: The [Rectangle] to check against.
    ///
    /// returns: bool - `true` if they overlap and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let rectangle1 = Rectangle::new([0, 0], [50, 50]);
    /// let rectangle2 = Rectangle::new([100, 100], [50, 50]);
    /// let rectangle3 = Rectangle::new([0, 0], [10, 10]);
    ///
    /// assert!(!rectangle1.collides(&rectangle2));
    /// assert!(rectangle1.collides(&rectangle3));
    /// assert!(!rectangle2.collides(&rectangle3));
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn collides(&self, other: &Rectangle) -> bool {
        self.left <= other.right
            && self.right >= other.left
            && self.bottom <= other.top
            && self.top >= other.bottom
    }

    /// Adds the given [Rectangle] the passed [TileMap] as a room the player can traverse.
    ///
    /// # Arguments
    ///
    /// * `map`: The [TileMap] to which the [Rectangle] should be added.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let map = GameMap::new([10, 10]);
    /// let room = Rectangle::new([0, 0], [5, 5]);
    ///
    /// room.add_to_map(&map);
    /// ```
    ///
    /// Mockup of the result:
    ///
    /// ```text
    /// ----------
    /// |        |
    /// |        |
    /// |#####   |
    /// |# . #   |
    /// |# . #   |
    /// |# . #   |
    /// |# . #   |
    /// |#####   |
    /// ----------
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    /// # Se also
    ///
    /// * [TileMap]
    /// * [MapTile]
    ///
    pub fn add_to_map(&self, map: &mut impl TileMap<MapTile>) {
        for x in self.left + 1..self.right {
            for y in self.bottom + 1..self.top {
                map.set_tile_at(&[x, y], MapTile::floor('.'));
            }
        }
    }

    /// Connects the given [Rectangle] and the passed one with corridors on the passed [TileMap].
    ///
    /// # Arguments
    ///
    /// * `other`: The [Rectangle] to which the calling one should be connect via corridors.
    /// * `map`: The [TileMap] on which the rooms are rendered.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// let map = GameMap::new([25, 25]);
    /// let room1 = Rectangle::new([0, 0], [5, 5]);
    /// let room2 = Rectangle::new([10, 10], [5, 5]);
    ///
    /// room.add_to_map(&map);
    /// room.add_to_map(&map);
    /// room1.connect(&room2, &map);
    ///
    /// ```
    ///
    /// Mockup of the result:
    ///
    /// ```text
    /// -------------------------
    /// |                 ##### |
    /// |                 # . # |
    /// |#####      ......... # |
    /// |# . #      .     # . # |
    /// |# ..........     # . # |
    /// |# . #            ##### |
    /// |# . #                  |
    /// |#####                  |
    /// -------------------------
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    /// # Se also
    ///
    /// * [TileMap]
    /// * [MapTile]
    ///
    pub fn connect(&self, other: &Rectangle, map: &mut impl TileMap<MapTile>) {
        let [x_end, y_end] = self.center();
        let [x_start, y_start] = other.center();

        for x in min(x_start, x_end)..=max(x_start, x_end) {
            map.set_tile_at(&[x, y_start], MapTile::floor('.'));
        }

        for y in min(y_start, y_end)..=max(y_start, y_end) {
            map.set_tile_at(&[x_end, y], MapTile::floor('.'));
        }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(({}, {}), ({}, {}))",
            self.left,
            self.bottom,
            self.right - self.left,
            self.top - self.bottom
        )
    }
}

impl Dimension2d for Rectangle {
    fn width(&self) -> i32 {
        self.left + self.right
    }

    fn height(&self) -> i32 {
        self.bottom + self.top
    }

    fn center(&self) -> [i32; 2] {
        [(self.left + self.right) / 2, (self.bottom + self.top) / 2]
    }
}

#[cfg(test)]
mod tests {
    use crate::core::dimension_2d::Dimension2d;
    use crate::ui::game_map::GameMap;
    use crate::ui::rectangle::Rectangle;
    use crate::ui::tile::Tile;
    use crate::ui::tile_map::TileMap;
    use crate::ui::tile_map_layout_generator::BaseTileMapGenerator;

    #[test]
    fn test_collision() {
        let rectangle1 = Rectangle::new([0, 0], [50, 50]);
        let rectangle2 = Rectangle::new([100, 100], [50, 50]);
        let rectangle3 = Rectangle::new([0, 0], [10, 10]);

        assert!(!rectangle1.collides(&rectangle2));
        assert!(rectangle1.collides(&rectangle3));
        assert!(!rectangle2.collides(&rectangle3));
    }

    #[test]
    fn rooms_are_added_to_map_correctly() {
        let mut map = GameMap::new(&[80, 50], &BaseTileMapGenerator);
        let rect = Rectangle::new([0, 0], [5, 5]);

        rect.add_to_map(&mut map);

        for x in 1..5 {
            for y in 1..5 {
                assert!(!map.get_tile_at(&[x, y]).has_collision())
            }
        }
    }

    #[test]
    fn rooms_are_connected_correctly() {
        let mut map = GameMap::new(&[80, 50], &BaseTileMapGenerator);
        let rect1 = Rectangle::new([0, 0], [5, 5]);
        let rect2 = Rectangle::new([6, 6], [5, 5]);

        rect1.add_to_map(&mut map);
        rect2.add_to_map(&mut map);

        rect1.connect(&rect2, &mut map);

        let [x_start, y_start] = rect1.center();
        let [x_end, y_end] = rect2.center();

        for x in x_start..x_end {
            assert!(!map.tile_has_collision(&[x, y_end]));
        }

        for y in y_start..y_end {
            assert!(!map.tile_has_collision(&[x_start, y]));
        }
    }
}
