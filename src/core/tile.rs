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

use bevy::prelude::{Color, Mut};
use bevy_ascii_terminal::{FormattedTile, Terminal, TileFormatter};

use crate::core::position_2d::Position2d;
use crate::core::view::View;

/// Defines all possible tiles which can be displayed on the in-game map.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
#[derive(Debug, Copy, Clone, Default)]
pub enum TileType {
    /// A standard, walkable tile, which makes up the default floor of the map.
    #[default]
    Floor,
}

impl View for TileType {
    fn render_at(&self, position: impl Position2d, terminal: &mut Mut<Terminal>) {
        match self {
            TileType::Floor => {
                terminal.put_char(position.as_array(), FormattedTile::new()
                    .glyph('.')
                    .fg(Color::GREEN)
                    .bg(Color::BLACK),
                )
            }
        }
    }
}
