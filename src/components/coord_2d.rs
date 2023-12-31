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

use bevy::prelude::Component;

use crate::core::position_2d::Position2d;

/// A positional [Component] describing the location of the associated `entity` in a
/// two dimensional space with its `x` and `y` values.
///
/// # Properties
///
/// * `x`: The location of the coordinate on the horizontal x-axis.
/// * `y`: The location of the coordinate on the vertical y-axis.
///
/// # Examples
///
/// ```
/// commands.spawn((
///     Coord2d::new(1, 1),
///     ascii_sprite!('@')
/// ));
///
/// ...
///
/// fn render_system(coord_query: Query<(Coord2d, AsciiSprite)>, terminal: Mut<Terminal>) {
///     for (coord, sprite) in coord_query.iter() {
///         terminal.put_char(coord.to_array(), sprite)
///     }
/// }
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct Coord2d {
    /// The location of the coordinate on the horizontal x-axis.
    pub x: i32,
    /// The location of the coordinate on the vertical y-axis.
    pub y: i32,
}

impl Coord2d {
    /// Creates a new [Coord2d] instance at the passed `x` and `y` coordinates.
    ///
    /// # Arguments
    ///
    /// * `x`: The location of the coordinate on the horizontal x-axis.
    /// * `y`: The location of the coordinate on the vertical y-axis.
    ///
    /// returns: [Coord2d]
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coord2d::new(1, 1);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Creates a new [Coord2d] instance based on the passed [Position2d].
    ///
    /// # Arguments
    ///
    /// * `position`: The [Position2d] based on which the [Coord2d] should be created.
    ///
    /// returns: [Coord2d]
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coord2d::from_position([1, 1]);
    ///
    /// assert_eq!(1, coordinate.x);
    /// assert_eq!(1, coordinate.y);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    /// # See also
    ///
    /// * [Position2d]
    ///
    pub fn from_position(position: &impl Position2d) -> Self {
        Coord2d::new(position.x_coordinate(), position.y_coordinate())
    }

    /// Returns the position above the coordinate on the vertical y-axis as a new
    /// [Coord2d] instance.
    ///
    /// If the y-position of the coordinate above would go over the passed `upper_bound`,
    /// the y-position of the returned [Coord2d] will be the `upper_bound`.
    ///
    /// # Arguments
    ///
    /// * `upper_bound`: The positive maximum for positions on the vertical y-axis.
    ///
    /// returns: [Coord2d]
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coord2d::new(1, 1);
    /// let above = coordinate.top(80, 50);
    /// let above_out_of_bounds = coordinate.down(1, 1);
    ///
    /// assert_eq!(2, above.y);
    /// assert_eq!(1, above_out_of_bounds.y);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn up(&self, upper_bound: i32) -> Self {
        Coord2d::new(self.x, min(self.y + 1, upper_bound))
    }

    /// Returns the position to the left on the horizontal x-axis of the coordinate as a new
    /// [Coord2d] instance.
    ///
    /// If the x-position of the coordinate to the left would go under the passed `lower_bound`,
    /// the x-position of the returned [Coord2d] will be the `lower_bound`.
    ///
    /// # Arguments
    ///
    /// * `lower_bound`: The negative maximum for positions on the horizontal x-axis.
    ///
    /// returns: [Coord2d]
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coord2d::new(1, 1);
    /// let left = coordinate.left(0, 0);
    /// let left_out_of_bounds = coordinate.right(1, 1);
    ///
    /// assert_eq!(0, left.x);
    /// assert_eq!(1, left_out_of_bounds.x);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn left(&self, lower_bound: i32) -> Self {
        Coord2d::new(max(self.x - 1, lower_bound), self.y)
    }

    /// Returns the position below the coordinate on the vertical y-axis as a new
    /// [Coord2d] instance.
    ///
    /// If the y-position of the coordinate below would go under the passed `lower_bound`,
    /// the y-position of the returned [Coord2d] will be the `lower_bound`.
    ///
    /// # Arguments
    ///
    /// * `lower_bound`: The negative maximum for positions on the vertical y-axis.
    ///
    /// returns: [Coord2d]
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coord2d::new(1, 1);
    /// let below = coordinate.down(0, 0);
    /// let below_out_of_bounds = coordinate.down(1, 1);
    ///
    /// assert_eq!(0, right.y);
    /// assert_eq!(1, below_out_of_bounds.y);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn down(&self, lower_bound: i32) -> Self {
        Coord2d::new(self.x, max(self.y - 1, lower_bound))
    }

    /// Returns the position to the right on the horizontal x-axis of the coordinate as a new
    /// [Coord2d] instance.
    ///
    /// If the x-position of the coordinate to the right would go over the passed `upper_bound`,
    /// the x-position of the returned [Coord2d] will be the `upper_bound`.
    ///
    /// # Arguments
    ///
    /// * `upper_bound`: The positive maximum for positions on the horizontal x-axis.
    ///
    /// returns: [Coord2d]
    ///
    /// # Examples
    ///
    /// ```
    /// let coordinate = Coord2d::new(1, 1);
    /// let right = coordinate.right(80, 50);
    /// let right_out_of_bounds = coordinate.right(1, 1);
    ///
    /// assert_eq!(2, right.x);
    /// assert_eq!(1, right_out_of_bounds.x);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn right(&self, upper_bound: i32) -> Self {
        Coord2d::new(min(self.x + 1, upper_bound), self.y)
    }
}

impl Display for Coord2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ECS -> Component -> Coord2d(x: {:?}, y: {:?})",
            self.x, self.y
        )
    }
}

impl Position2d for Coord2d {
    fn x_coordinate(&self) -> i32 {
        self.x
    }

    fn y_coordinate(&self) -> i32 {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use crate::components::coord_2d::Coord2d;

    #[test]
    fn test_interoperability_with_position_2d() {
        let coord2d = Coord2d::from_position(&[40.0, 25.0]);

        assert_eq!(40, coord2d.x);
        assert_eq!(25, coord2d.y);
    }

    #[test]
    fn test_top_coordinate_calculation() {
        let coord2d = Coord2d::new(1, 1);

        let up_in_bounds = coord2d.up(2);
        let up_out_of_bounds = coord2d.up(0);

        assert_eq!(2, up_in_bounds.y);
        assert_eq!(0, up_out_of_bounds.y);
    }

    #[test]
    fn test_left_coordinate_calculation() {
        let coord2d = Coord2d::new(1, 1);

        let left_in_bounds = coord2d.left(0);
        let left_out_of_bounds = coord2d.left(1);

        assert_eq!(0, left_in_bounds.x);
        assert_eq!(1, left_out_of_bounds.x);
    }

    #[test]
    fn test_down_coordinate_calculation() {
        let coord2d = Coord2d::new(1, 1);

        let down_in_bounds = coord2d.down(0);
        let down_out_of_bounds = coord2d.down(1);

        assert_eq!(0, down_in_bounds.y);
        assert_eq!(1, down_out_of_bounds.y);
    }

    #[test]
    fn test_right_coordinate_calculation() {
        let coord2d = Coord2d::new(1, 1);

        let right_in_bounds = coord2d.right(2);
        let right_out_of_bounds = coord2d.right(1);

        assert_eq!(2, right_in_bounds.x);
        assert_eq!(1, right_out_of_bounds.x);
    }
}
