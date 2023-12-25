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

use bevy::prelude::Component;

use crate::components::coord_2d::Coord2d;
use crate::core::position_2d::Position2d;

/// # Properties
///
/// * `radius`:
/// * `is_dirty`:
/// * `coordinates`:
///
/// # Examples
///
/// ```
///
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
#[derive(Debug, Component)]
pub struct Fov {
    pub radius: i32,
    pub is_dirty: bool,
    coordinates: Vec<Coord2d>,
}

impl Fov {
    ///
    ///
    /// # Arguments
    ///
    /// * `radius`:
    ///
    /// returns: [Fov]
    ///
    /// # Examples
    ///
    /// ```
    /// let fov = Fov::new(8);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn new(radius: i32) -> Self {
        Self {
            radius,
            is_dirty: true,
            coordinates: Vec::new(),
        }
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `position`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn mark_position_as_visible(&mut self, position: &impl Position2d) {
        self.coordinates.push(Coord2d::from_position(position));
    }

    /// Removes all [Coord2d]s currently in the field of view, making it empty.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn clear(&mut self) {
        self.coordinates.clear();
    }
}
