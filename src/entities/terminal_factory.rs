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

use bevy::prelude::Commands;
use bevy_ascii_terminal::{Terminal, TerminalBundle, TerminalFont, TileScaling, TiledCameraBundle};
use std::fmt::{Display, Formatter};

use crate::components::game_terminal::GameTerminal;
use crate::core::dimension_2d::Dimension2d;

/// Factory to create the tile and terminal based `entities`.
///
/// This includes
/// * [Terminal]
/// * [TerminalBundle]
/// * [TiledCameraBundle]
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.8`
///
#[derive(Debug)]
pub struct TerminalFactory;

impl TerminalFactory {
    /// Sets up and creates the tile based UI components for the game, this includes
    /// * The required [Terminal]s
    /// * The corresponding [TerminalBundle]
    /// * The camera of the game through a [TiledCameraBundle]
    ///
    /// # Arguments
    ///
    /// * `commands`: [Commands] queue required to spawn the `entities`.
    /// * `font`: The [TerminalFont] to use for rendered glyphs.
    /// * `screen_size`: The size of the area, which the terminals should take up.
    ///
    /// returns: ()
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    pub fn spawn(commands: &mut Commands, font: TerminalFont, screen_size: &impl Dimension2d) {
        let tile_count = screen_size.as_array();

        commands
            .spawn(
                TerminalBundle::from(Terminal::new(tile_count))
                    .with_tile_scaling(TileScaling::World)
                    .with_font(font),
            )
            .insert(GameTerminal);

        commands.spawn(TiledCameraBundle::new().with_tile_count(tile_count));
    }
}

impl Display for TerminalFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ECS -> Entities -> TerminalFactory")
    }
}
