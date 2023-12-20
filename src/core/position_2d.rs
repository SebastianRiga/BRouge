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

use bevy::math::{IVec2, UVec2, Vec2};
use std::fmt::Debug;

/// Describes a position in a two dimensional state, consisting of a `x` and `y``coordinate.
///
/// The trait is implemented by the default for the following types:
///
/// * [Vec2]
/// * [IVec2]
/// * [UVec2]
/// * `[u32; 2]`
/// * `[i32; 2]`
/// * `[f32; 2]`
/// * `[usize; 2]`
///
/// # Examples
///
/// ```
/// fn take_position(position2d: impl Position2d) { ... }
///
/// take_position([3, 4]);
/// take_position([1.0, 2.0]);
/// take_position(IVec2::new(5, 3));
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
/// * [crate::components::coord_2d::Coord2d]
///
pub trait Position2d: Debug + Copy + Clone {
    /// The location on the horizontal x-axis of the [Position2d].
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn x(&self) -> i32;

    /// The location on the vertical y-axis of the [Position2d].
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn y(&self) -> i32;

    /// Creates a new `i32` array with a fixed length of `2`, which contains the [Position2d]'s `x` coordinate
    /// at the first index and the `y` coordinate at the last.
    ///
    /// # Examples
    ///
    /// ```
    /// let position2d = Vec2::new(2.0, 3.0);
    ///
    /// assert_eq!([2, 3], position2d.to_array());
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn as_array(&self) -> [i32; 2] {
        [self.x(), self.y()]
    }
}

/// Internal macro to generate the [Position2d] trait implementations for existing types.
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
macro_rules! implement_position_2d {
    ($type:ty) => {
        impl Position2d for $type {
            fn x(&self) -> i32 {
                self[0] as i32
            }

            fn y(&self) -> i32 {
                self[1] as i32
            }
        }
    };
}

implement_position_2d!(Vec2);
implement_position_2d!(IVec2);
implement_position_2d!(UVec2);
implement_position_2d!([u32; 2]);
implement_position_2d!([i32; 2]);
implement_position_2d!([f32; 2]);
implement_position_2d!([usize; 2]);

#[cfg(test)]
mod tests {
    use super::*;

    const VEC2: Vec2 = Vec2::new(80.0, 50.0);
    const I_VEC2: IVec2 = IVec2::new(80, 50);
    const U_VEC2: UVec2 = UVec2::new(80, 50);
    const U32_ARRAY: [u32; 2] = [80u32, 50u32];
    const I32_ARRAY: [i32; 2] = [80, 50];
    const F32_ARRAY: [f32; 2] = [80.0f32, 50.0f32];
    const USIZE_ARRAY: [usize; 2] = [80usize, 50usize];

    #[test]
    fn test_computed_properties() {
        assert_eq!(80, VEC2.x());
        assert_eq!(80, I_VEC2.x());
        assert_eq!(80, U_VEC2.x());
        assert_eq!(80, U32_ARRAY.x());
        assert_eq!(80, I32_ARRAY.x());
        assert_eq!(80, F32_ARRAY.x());
        assert_eq!(80, USIZE_ARRAY.x());

        assert_eq!(50, VEC2.y());
        assert_eq!(50, I_VEC2.y());
        assert_eq!(50, U_VEC2.y());
        assert_eq!(50, U32_ARRAY.y());
        assert_eq!(50, I32_ARRAY.y());
        assert_eq!(50, F32_ARRAY.y());
        assert_eq!(50, USIZE_ARRAY.y());
    }

    //noinspection ALL
    #[test]
    fn test_array_conversion() {
        assert_eq!([80, 50], VEC2.as_array());
        assert_eq!([80, 50], I_VEC2.as_array());
        assert_eq!([80, 50], U_VEC2.as_array());
        assert_eq!([80, 50], U32_ARRAY.as_array());
        assert_eq!([80, 50], I32_ARRAY.as_array());
        assert_eq!([80, 50], F32_ARRAY.as_array());
        assert_eq!([80, 50], USIZE_ARRAY.as_array());
    }
}
