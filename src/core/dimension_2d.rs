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

use bevy::math::{IVec2, UVec2};
use bevy::prelude::Vec2;

use crate::core::position_2d::Position2d;

/// Describes a two dimensional area defined by a horizontal width and a vertical height.
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
/// fn take_dimension(position2d: impl Dimension2d) { ... }
///
/// take_dimension([3, 4]);
/// take_dimension([1.0, 2.0]);
/// take_dimension(IVec2::new(5, 3));
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
pub trait Dimension2d: Debug + Clone {
    /// The horizontal width of the dimension.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn width(&self) -> i32;

    /// The vertical height of the dimension.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn height(&self) -> i32;

    /// Returns the center coordinate of the area in form a of an `array` with the fixed length of `2`.
    /// The resulting `array` contains the center `x` coordinate at the first index
    /// and the center `y` coordinate at the last.
    ///
    /// # Examples
    ///
    /// ```
    /// let dimension2d = [80, 50];
    ///
    /// assert_eq!([40, 25], dimension2d.center());
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
    fn center(&self) -> [i32; 2] {
        [self.width() / 2, self.height() / 2]
    }

    /// Calculates the area of the dimension as a `usize`.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn area(&self) -> usize {
        (self.width() * self.height()) as usize
    }

    /// Checks if the passed `position` is within the bounds of this area.
    ///
    /// # Arguments
    ///
    /// * `position`: The position to bounds-check.
    ///
    /// returns: [bool] - `true` if the passed `position` is in bounds and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let dimension = [400, 200];
    ///
    /// assert!(dimension.is_in_bounds([24, 18]));
    /// assert!(!dimension.is_in_bounds([500, 250]));
    /// assert!(!dimension.is_in_bounds([-2, -30]));
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    fn is_in_bounds(&self, position: &impl Position2d) -> bool {
        (0..self.width() - 1).contains(&position.x_coordinate())
            && (0..self.height() - 1).contains(&position.y_coordinate())
    }

    /// Creates a new `i32` array with a fixed length of `2`, which contains the [Dimension2d]'s
    /// width at the first position and the height at the last.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn as_array(&self) -> [i32; 2] {
        [self.width(), self.height()]
    }
}

/// Internal macro to generate the [Dimension2d] trait implementations for existing types.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
macro_rules! implement_dimension_2d {
    ($type:ty) => {
        impl Dimension2d for $type {
            fn width(&self) -> i32 {
                self[0] as i32
            }

            fn height(&self) -> i32 {
                self[1] as i32
            }
        }
    };
}

implement_dimension_2d!(Vec2);
implement_dimension_2d!(IVec2);
implement_dimension_2d!(UVec2);
implement_dimension_2d!([i32; 2]);
implement_dimension_2d!([f32; 2]);
implement_dimension_2d!([u32; 2]);
implement_dimension_2d!([usize; 2]);

#[cfg(test)]
mod tests {
    use crate::core::dimension_2d::Dimension2d;
    use bevy::prelude::IVec2;
    use bevy::prelude::*;

    const VEC2: Vec2 = Vec2::new(80.0, 50.0);
    const I_VEC2: IVec2 = IVec2::new(80, 50);
    const U_VEC2: UVec2 = UVec2::new(80, 50);
    const U32_ARRAY: [u32; 2] = [80u32, 50u32];
    const I32_ARRAY: [i32; 2] = [80, 50];
    const F32_ARRAY: [f32; 2] = [80.0f32, 50.0f32];
    const USIZE_ARRAY: [usize; 2] = [80usize, 50usize];

    #[test]
    fn test_computed_properties() {
        assert_eq!(80, VEC2.width());
        assert_eq!(80, I_VEC2.width());
        assert_eq!(80, U_VEC2.width());
        assert_eq!(80, U32_ARRAY.width());
        assert_eq!(80, I32_ARRAY.width());
        assert_eq!(80, F32_ARRAY.width());
        assert_eq!(80, USIZE_ARRAY.width());

        assert_eq!(50, VEC2.height());
        assert_eq!(50, I_VEC2.height());
        assert_eq!(50, U_VEC2.height());
        assert_eq!(50, U32_ARRAY.height());
        assert_eq!(50, I32_ARRAY.height());
        assert_eq!(50, F32_ARRAY.height());
        assert_eq!(50, USIZE_ARRAY.height());
    }

    #[test]
    fn test_center_calculation() {
        assert_eq!([40, 25], VEC2.center());
        assert_eq!([40, 25], I_VEC2.center());
        assert_eq!([40, 25], U_VEC2.center());
        assert_eq!([40, 25], U32_ARRAY.center());
        assert_eq!([40, 25], I32_ARRAY.center());
        assert_eq!([40, 25], F32_ARRAY.center());
        assert_eq!([40, 25], USIZE_ARRAY.center());
    }

    #[test]
    fn test_area_calculation() {
        assert_eq!(4000, VEC2.area());
        assert_eq!(4000, I_VEC2.area());
        assert_eq!(4000, U_VEC2.area());
        assert_eq!(4000, U32_ARRAY.area());
        assert_eq!(4000, I32_ARRAY.area());
        assert_eq!(4000, F32_ARRAY.area());
        assert_eq!(4000, USIZE_ARRAY.area());
    }

    #[test]
    fn test_is_in_bounds_check() {
        let dimension = [400, 200];

        assert!(dimension.is_in_bounds(&[0, 0]));
        assert!(dimension.is_in_bounds(&[24, 58]));
        assert!(!dimension.is_in_bounds(&[500, 300]));
        assert!(!dimension.is_in_bounds(&[-2, -300]));
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
