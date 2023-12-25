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

use crate::components::game_map::GameMap;
use crate::core::dimension_2d::Dimension2d;
use crate::core::position_2d::Position2d;
use crate::core::tile::TileType;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
    pub top: i32,
}

impl Rectangle {
    pub fn new(origin: impl Position2d, dimension: impl Dimension2d) -> Self {
        Self {
            left: origin.x_coordinate(),
            bottom: origin.y_coordinate(),
            right: origin.x_coordinate() + dimension.width(),
            top: origin.y_coordinate() + dimension.height(),
        }
    }

    pub fn collides(&self, other: &Rectangle) -> bool {
        self.left <= other.right
            && self.right >= other.left
            && self.bottom <= other.top
            && self.top >= other.bottom
    }

    pub fn add_to_map(&self, map: &mut GameMap) {
        for x in self.left + 1..self.right {
            for y in self.bottom + 1..self.top {
                map.set_tile_at(&[x, y], TileType::Floor);
            }
        }

        map.rooms.push(*self)
    }

    pub fn connect(&self, other: &Rectangle, map: &mut GameMap) {
        let [x_end, y_end] = self.center();
        let [x_start, y_start] = other.center();

        for x in min(x_start, x_end)..=max(x_start, x_end) {
            map.set_tile_at(&[x, y_start], TileType::Floor);
        }

        for y in min(y_start, y_end)..=max(y_start, y_end) {
            map.set_tile_at(&[x_end, y], TileType::Floor);
        }
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

}
