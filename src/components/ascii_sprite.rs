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

use bevy::prelude::{Color, Component, Mut};
use bevy_ascii_terminal::{Terminal, TileFormatter};

use crate::core::position_2d::Position2d;
use crate::core::var_args::VarArgs;
use crate::core::view::View;

/// [Component] marking an `entity` as renderable sprite of the game, made up of an ascii symbol,
/// a foreground and background color.
///
/// # Properties
///
/// * `glyph`: The ascii symbol to use when rendering the `entity` on screen, e.g., '@'.
/// * `foreground_color`: The foreground color to use when rendering the `entity` on screen.
/// * `background_color`: The background color to use when rendering the `entity` on screen.
///
/// # Macros
///
/// The [AsciiSprite] provides the [crate::ascii_sprite] macro to simplify the creation of sprites,
/// by providing defaults for the foreground and background colors.
///
/// # Examples:
///
/// ```
/// commands.spawn((
///     Player,
///     Coord2d::from_position(starting_position),
///     ascii_sprite!('@', Color::ORANGE, Color::BLACK) // Using the macro to create the sprite
/// ))
/// ...
///
/// fn render_system(&mut terminal: Terminal, sprite_query: Query<AsciiSprite>) {
///     for sprite in sprite_query.iter() {
///         sprite.render_at([1, 1], terminal);
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
/// # See also
///
/// * [View]
/// * [crate::ascii_sprite]
///
#[derive(Debug, Copy, Clone, PartialEq, Component)]
pub struct AsciiSprite {
    /// The ascii symbol to use when rendering the `entity` on screen, e.g., '@'.
    pub glyph: char,
    /// The foreground color to use when rendering the `entity` on screen.
    pub foreground_color: Color,
    /// The background color to use when rendering the `entity` on screen.
    pub background_color: Color,
}

impl AsciiSprite {
    /// Creates a new [AsciiSprite] [Component] with the passed `glyph`, `foreground_color` and
    /// `background_color`.
    ///
    /// # Arguments
    ///
    /// * `glyph`: The ascii symbol to use when rendering the `entity` on screen, e.g., '@'.
    /// * `foreground_color`: The foreground color to use when rendering the `entity` on screen.
    /// * `background_color`: The background color to use when rendering the `entity` on screen.
    ///
    /// returns: [AsciiSprite]
    ///
    /// # Examples
    ///
    /// ```
    /// let ascii_sprite = AsciiSprite::new('@', Color::Orange, Color::Black);
    ///
    /// commands.spawn((..., ascii_sprite, ...));
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
    /// * `ascii_sprite` macro
    ///
    pub fn new(glyph: char, foreground_color: Color, background_color: Color) -> Self {
        Self {
            glyph,
            foreground_color,
            background_color,
        }
    }
}

impl View for AsciiSprite {
    fn render_at(&self, position: &impl Position2d, terminal: &mut Mut<Terminal>, _options: &VarArgs) {
        terminal.put_char(
            position.as_array(),
            self.glyph
                .fg(self.foreground_color)
                .bg(self.background_color),
        )
    }
}

/// Macro to simplify the creation of [AsciiSprite]s, by providing defaults for the
/// foreground and background colors.
///
/// Expands to the [AsciiSprite::new] constructor function.
///
/// # Arguments
///
/// * `glyph`: The ascii symbol to use when rendering the `entity` on screen, e.g., '@'.
/// * (Optional) `foreground_color`: The foreground color to use when rendering the `entity`
/// on screen, defaults to [Color::WHITE].
/// * (Optional) `background_color`: The background color to use when rendering the `entity`
/// on screen, defaults to [Color::BLACK].
///
/// returns: [AsciiSprite]
///
/// # Examples
///
/// ```
/// let standard_sprite = ascii_sprite('@');
/// let sprite_with_custom_foreground = ascii_sprite('A', Color::ORANGE);
/// let sprite_with_custom_foreground_and_background = ascii_sprite('B', Color::GREEN, Color::RED);
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
/// * [AsciiSprite]
///
#[macro_export]
macro_rules! ascii_sprite {
    ($glyph: expr) => {
        $crate::components::ascii_sprite::AsciiSprite::new(
            $glyph,
            bevy::prelude::Color::WHITE,
            bevy::prelude::Color::BLACK,
        )
    };
    ($glyph: expr, $foreground_color: expr) => {
        $crate::components::ascii_sprite::AsciiSprite::new(
            $glyph,
            $foreground_color,
            bevy::prelude::Color::WHITE,
        )
    };
    ($glyph: expr, $foreground_color: expr, $background_color: expr) => {
        $crate::components::ascii_sprite::AsciiSprite::new(
            $glyph,
            $foreground_color,
            $background_color,
        )
    };
}
