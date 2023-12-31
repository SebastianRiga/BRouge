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

use std::fmt::{Display, Formatter};

use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::rngs::ThreadRng;
use rand::Rng;

/// An OS based random number generator, which provides functionality to pick random values from ranges and roll
/// classic D&D style dice.
///
/// This struct is not thread safe!
///
/// # Examples
///
/// ```
/// let rng = RandomNumberGenerator::new();
///
/// let mut result = rng.range(0..2);
///
/// // Will log either 0 or 1
/// info!("Result: {:?}", result);
///
/// result = rng.range(0..=2);
///
/// // Will log either 0, 1 or 2.
/// info!("Result: {:?}", result);
///
/// // This rolls a classic 3d6 dice.
/// rng.roll_dice(3, 6);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
/// # See also
///
/// * [ThreadRng]
///
#[derive(Debug)]
pub struct RandomNumberGenerator {
    generator: ThreadRng,
}

impl RandomNumberGenerator {
    /// Creates a new [RandomNumberGenerator] instance based on a OS reliant seed.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn new() -> Self {
        Self {
            generator: rand::thread_rng(),
        }
    }

    /// Picks a random value from the set `range`.
    ///
    /// Only upper bound inclusive and exclusive ranges are supported, e.g.
    /// * 0..2 -> Picks either 0 or 1.
    /// * 0..=2 -> Picks either 0, 1, or 2.
    ///
    /// # Parameters
    ///
    /// * T: The [SampleUniform] type to use for the `range` and the return value, e.g., i32, f32, usize, etc.
    /// * R: The [SampleRange] type from which a random value will be picked.
    ///
    /// # Arguments
    ///
    /// * `range`: The range from which a random value will be picked.
    ///
    /// returns: T
    ///
    /// # Examples
    ///
    /// ```
    /// let rng = RandomNumberGenerator::new();
    ///
    /// let mut result = rng.range(0..2);
    ///
    /// // Will log either 0 or 1
    /// info!("Result: {:?}", result);
    ///
    /// result = rng.range(0..=2);
    ///
    /// // Will log either 0, 1 or 2.
    /// info!("Result: {:?}", result);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn range<R, T>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.generator.gen_range(range)
    }

    /// Rolls a dice with the passed amount of `faces` the given `number` of times and adds the results of each dice
    /// roll together.
    ///
    /// # Arguments
    ///
    /// * `number`: The number of times the dice should be rolled.
    /// * `faces`: The number of faces the dice has.
    ///
    /// returns: i32
    ///
    /// # Examples
    ///
    /// ```
    /// let mut rng = RandomNumberGenerator::new();
    ///
    /// // This rolls a classic 3d6 dice.
    /// rng.roll_dice(3, 6);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.7`
    ///
    pub fn roll_dice(&mut self, number: i32, faces: i32) -> i32 {
        let mut sum = 0;

        for _ in 0..number {
            sum += self.generator.gen_range(1..=faces)
        }

        sum
    }
}

impl Display for RandomNumberGenerator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RandomNumberGenerator(generator: {:?}))", self.generator)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::rng::RandomNumberGenerator;

    #[test]
    fn test_dice_rolls() {
        let mut rng = RandomNumberGenerator::new();

        let one_d_one = rng.roll_dice(1, 1);
        let three_d_six = rng.roll_dice(3, 6);
        let ten_d_twelve = rng.roll_dice(10, 12);

        assert_eq!(1, one_d_one);
        assert!(three_d_six >= 3 && three_d_six <= 18);
        assert!(ten_d_twelve >= 10 && ten_d_twelve <= 120);
    }
}
