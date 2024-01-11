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

use std::fmt::{Debug, Display, Formatter};

use bevy::prelude::Component;

use crate::core::position_2d::Position2d;

/// [Component] tracking the [Coord2d] based positions currently in the `field of view` of the associated `entity`.
///
/// # Properties
///
/// * `radius`: The radius of the `field of view`.
/// * `is_dirty`: If the `field of view` needs to be recalculated.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
#[derive(Clone, Eq, PartialEq, Hash, Component)]
pub struct Fov {
    /// The radius of the `field of view`.
    pub radius: i32,
    /// If the `field of view` needs to be recalculated.
    pub is_dirty: bool,
    /// (Private) List of tuple based [Position2d]s currently in the `field of view`.
    coordinates: Vec<(i32, i32)>,
}

impl Fov {
    /// Creates a new [Fov] instance with the passed `radius`.
    ///
    /// The new instance's `is_dirty` flag is initially set to `true`, in order to trigger an immediate calculation.
    ///
    /// # Arguments
    ///
    /// * `radius`: The radius of the `field of view`.
    ///
    /// returns: [Fov]
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
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.9`
    ///
    pub fn positions(&self) -> &Vec<impl Position2d> {
        &self.coordinates
    }

    /// Adds the passed `position` to the [Fov], marking it as in the `field of view`
    /// of the associated `entity`.
    ///
    /// # Arguments
    ///
    /// * `position`: The position to add.
    ///
    /// returns: ()
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn push_position(&mut self, position: &impl Position2d) {
        self.coordinates.push(position.as_tuple());
    }

    /// Checks if the passed [Position2d] is in the `field of view`.
    ///
    /// # Arguments
    ///
    /// * `position`: The position to check.
    ///
    /// returns: bool - `true` if the `position` is in the `field of view` and `false` otherwise.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.9`
    ///
    pub fn contains(&self, position: &impl Position2d) -> bool {
        self.coordinates.contains(&position.as_tuple())
    }

    /// Removes all [Coord2d]s currently in the field of view, making it empty.
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

impl Debug for Fov {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ECS -> Components -> Fov {{ \
        radius: {:?}, \
        is_dirty: {:?}, \
        coordinates: {:?} \
        }}",
            self.radius, self.is_dirty, self.coordinates
        )
    }
}

impl Display for Fov {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.radius,
            self.is_dirty,
            self.coordinates.len()
        )
    }
}
