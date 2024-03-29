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

/// A [Component] for naming the associated entity.
///
/// # Properties
///
/// * `text`:
///
/// # Examples
///
/// ```
/// commands.spawn((
///     ...,
///     NameTag::new("Player");
///     ...,
/// ));
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.9`
///
#[derive(Clone, Eq, PartialEq, Hash, Component)]
pub struct NameTag {
    pub text: String,
}

impl NameTag {
    /// Creates a new [NameTag] [Component] instance with passed `text`.
    ///
    /// # Arguments
    ///
    /// * `text`: The name to use for the associated `entity`.
    ///
    /// returns: [NameTag]
    ///
    /// # Examples
    ///
    /// ```
    /// let name_tag = NameTag::new("Player");
    ///
    /// assert_eq!("Player", name_tag.text);
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.9`
    ///
    pub fn new(text: &str) -> Self {
        Self {
            text: String::from(text),
        }
    }
}

impl Debug for NameTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ECS -> Components -> NameTag {{ text: {:?} }}",
            self.text
        )
    }
}

impl Display for NameTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.text)
    }
}
