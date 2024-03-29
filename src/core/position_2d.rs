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

use std::fmt::Debug;

use bevy::math::{IVec2, UVec2, Vec2};

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
/// * `(i32, i32)`
/// * `(i32, i32)`
/// * `(f32, f32)`
/// * `(usize, usize)`
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
pub trait Position2d: Debug + Copy + Clone + PartialEq {
    /// The location on the horizontal x-axis of the [Position2d].
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn x_coordinate(&self) -> i32;

    /// The location on the vertical y-axis of the [Position2d].
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn y_coordinate(&self) -> i32;

    /// Calculates the delta between this and the passed `other` position by subtracting its coordinates
    /// from the calling ones.
    ///
    /// # Arguments
    ///
    /// * `other`: The [Position2d] with which the delta should be calculated.
    ///
    /// returns: [i32; 2]
    ///
    /// # Examples
    ///
    /// ```
    /// let start = [8, 4];
    /// let end = [6, 3];
    ///
    /// assert_eq!([2, 1], start.delta(&end).as_array());
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    fn delta(&self, other: &impl Position2d) -> [i32; 2] {
        [
            self.x_coordinate() - other.x_coordinate(),
            self.y_coordinate() - other.y_coordinate(),
        ]
    }

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
        [self.x_coordinate(), self.y_coordinate()]
    }

    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.9`
    ///
    fn as_tuple(&self) -> (i32, i32) {
        (self.x_coordinate(), self.y_coordinate())
    }
}

/// Internal macro to generate the [Position2d] trait implementations for existing array index-able types.
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
macro_rules! implement_position_2d_for_array {
    ($type:ty) => {
        impl Position2d for $type {
            fn x_coordinate(&self) -> i32 {
                self[0] as i32
            }

            fn y_coordinate(&self) -> i32 {
                self[1] as i32
            }
        }
    };
}

implement_position_2d_for_array!(Vec2);
implement_position_2d_for_array!(IVec2);
implement_position_2d_for_array!(UVec2);
implement_position_2d_for_array!([u32; 2]);
implement_position_2d_for_array!([i32; 2]);
implement_position_2d_for_array!([f32; 2]);
implement_position_2d_for_array!([usize; 2]);

/// Internal macro to generate the [Position2d] trait implementations for tuple types.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.9`
///
/// # See also
///
/// * [Position2d]
///
macro_rules! implement_position_2d_for_tuple {
    ($type:ty) => {
        impl Position2d for $type {
            fn x_coordinate(&self) -> i32 {
                self.0 as i32
            }

            fn y_coordinate(&self) -> i32 {
                self.1 as i32
            }
        }
    };
}

implement_position_2d_for_tuple!((u32, u32));
implement_position_2d_for_tuple!((i32, i32));
implement_position_2d_for_tuple!((f32, f32));
implement_position_2d_for_tuple!((usize, usize));

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
    const U_TUPLE: (u32, u32) = (80u32, 50u32);
    const I_TUPLE: (i32, i32) = (80, 50);
    const F_TUPLE: (f32, f32) = (80.0f32, 50.0f32);
    const USIZE_TUPLE: (usize, usize) = (80usize, 50usize);

    #[test]
    fn test_computed_properties() {
        assert_eq!(80, VEC2.x_coordinate());
        assert_eq!(80, I_VEC2.x_coordinate());
        assert_eq!(80, U_VEC2.x_coordinate());
        assert_eq!(80, U32_ARRAY.x_coordinate());
        assert_eq!(80, I32_ARRAY.x_coordinate());
        assert_eq!(80, F32_ARRAY.x_coordinate());
        assert_eq!(80, USIZE_ARRAY.x_coordinate());
        assert_eq!(80, U_TUPLE.x_coordinate());
        assert_eq!(80, I_TUPLE.x_coordinate());
        assert_eq!(80, F_TUPLE.x_coordinate());
        assert_eq!(80, USIZE_TUPLE.x_coordinate());

        assert_eq!(50, VEC2.y_coordinate());
        assert_eq!(50, I_VEC2.y_coordinate());
        assert_eq!(50, U_VEC2.y_coordinate());
        assert_eq!(50, U32_ARRAY.y_coordinate());
        assert_eq!(50, I32_ARRAY.y_coordinate());
        assert_eq!(50, F32_ARRAY.y_coordinate());
        assert_eq!(50, USIZE_ARRAY.y_coordinate());
        assert_eq!(50, U_TUPLE.y_coordinate());
        assert_eq!(50, I_TUPLE.y_coordinate());
        assert_eq!(50, F_TUPLE.y_coordinate());
        assert_eq!(50, USIZE_TUPLE.y_coordinate());
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
        assert_eq!([80, 50], U_TUPLE.as_array());
        assert_eq!([80, 50], I_TUPLE.as_array());
        assert_eq!([80, 50], F_TUPLE.as_array());
        assert_eq!([80, 50], USIZE_TUPLE.as_array());
    }
}
