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

use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::rngs::ThreadRng;
use rand::Rng;

///
/// # Properties
///
/// * `generator`:
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
#[derive(Debug)]
pub struct RandomNumberGenerator {
    generator: ThreadRng,
}

impl RandomNumberGenerator {
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

    ///
    ///
    /// # Arguments
    ///
    /// * `range`:
    ///
    /// returns: T
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
    pub fn range<R, T>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.generator.gen_range(range)
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `number`:
    /// * `faces`:
    ///
    /// returns: i32
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
    pub fn roll_dice(&mut self, number: i32, faces: i32) -> i32 {
        let mut sum = 0;

        for _ in 0..number {
            sum += self.generator.gen_range(1..=faces)
        }

        sum
    }
}
