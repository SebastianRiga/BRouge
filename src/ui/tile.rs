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

use std::fmt;
use std::fmt::{Display, Formatter};

use bevy::prelude::{Color, Mut};
use bevy_ascii_terminal::{Terminal, TileFormatter};

use crate::core::position_2d::Position2d;
use crate::ui::colors;

/// A singular tile instance which can be rendered on demand given a specific position, usually as an isolated
/// part owned by a [TileMap]. The map supplies supplies the position in its respective [TileMap::render] function.
///
/// Each [Tile] is responsible for it's visual representation, e.g., markup, foreground and background,
/// only the actual render position within the game's world is passed in from the outside.
///
/// # Examples
///
/// ```
/// pub struct TileImpl {
///     pub glyph: char,
///     pub fg: Color,
///     pub bg: Color,
/// }
///
/// impl Tile for TileImpl {
///     ...
/// }
///
/// ...
///
/// struct MapImpl<TileImpl> {
///     pub tiles: Vec<TileImpl>,
///     pub visible_tiles: Vec<bool>,
///     pub seen_tiles: Vec<bool>,
/// }
///
/// impl TileMap for MapImpl {
///     fn render(&self, terminal: &mut Mut<Terminal>) {
///         for x in 0..80 {
///             for < in 0..50 {
///                 let world_index = Self::convert_world_index(80, [x, y]);
///                 self.tiles[world_index].render(
///                     [x, y],
///                     terminal,
///                     self.seen_tiles[world_index],
///                     self.visible_tiles[world_index]
///                 );
///             }
///         }
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
/// * [Terminal]
/// * [TileMap]
/// * [Position2d]
///
pub trait Tile {
    /// The glyph used to render the given [Tile] on the [TileMap], e.g.
    /// * `@`
    /// * `.`
    /// * `#`
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    fn glyph(&self) -> char;

    /// The foreground color to use for the [Tile] when rendering it on the [TileMap].
    ///
    /// # Arguments
    ///
    /// * `is_seen`: If the [Tile] has been seen by the player before.
    /// * `is_visible`: If the [Tile] is in the `field of view` of the `player`.
    ///
    /// returns: Color
    ///
    /// # Examples
    ///
    /// ```
    /// fn foreground_color(&self, _is_seen: bool, is_visible: bool) -> Color {
    ///    if is_visible {
    ///        Color::SEA_GREEN
    ///    } else {
    ///        colors::INACTIVE
    ///    }
    ///}
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    fn foreground_color(&self, is_seen: bool, is_visible: bool) -> Color;

    /// The background color to use for the [Tile] when rendering it on the [TileMap].
    ///
    /// # Arguments
    ///
    /// * `is_seen`: If the [Tile] has been seen by the `player` before.
    /// * `is_visible`: If the [Tile] is in the `field of view` of the `player`.
    ///
    /// returns: Color
    ///
    /// # Examples
    ///
    /// ```
    /// fn background_color(&self, is_seen: bool, _is_visible: bool) -> Color {
    ///    if is_seen {
    ///        Color::WHITE
    ///    } else {
    ///        colors::BACKGROUND
    ///    }
    ///}
    /// ```
    fn background_color(&self, is_seen: bool, is_visible: bool) -> Color;

    /// If actors, e.g., the player, monsters, items, etc., can be placed on the [Tile], or if it blocks
    /// the space it occupies.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    fn has_collision(&self) -> bool;

    /// Renders the [Tile] at the given `position` using the passed `terminal` reference.
    ///
    /// # Arguments
    ///
    /// * `position`: The [Position2d] to render the [Tile] at.
    /// * `terminal`: [Terminal] which handles the actual rendering.
    /// * `is_seen`: If the [Tile] has been seen by the `player` before.
    /// * `is_visible`: If the [Tile] is in the `field of view` of the `player`.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Component)]
    /// pub struct TileImpl {
    ///     pub glyph: char,
    ///     pub fg: Color,
    ///     pub bg: Color,
    /// }
    ///
    /// impl Tile for TileImpl {
    ///     ...
    /// }
    ///
    /// ...
    ///
    /// fn render_system(tile_query: Query<TileImpl>, &mut terminal: Terminal) {
    ///     for tile in tile_query.iter() {
    ///         tile.render_at([x, y], terminal, true, true);
    ///     }
    /// }
    ///
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.15`
    ///
    /// # See also
    ///
    /// * [Terminal]
    /// * [Position2d]
    ///
    fn render(
        &self,
        position: &impl Position2d,
        terminal: &mut Mut<Terminal>,
        is_seen: bool,
        is_visible: bool,
    ) {
        if is_seen || is_visible {
            terminal.put_char(
                position.as_array(),
                self.glyph()
                    .fg(self.foreground_color(is_seen, is_visible))
                    .bg(self.background_color(is_seen, is_visible)),
            );
        }
    }
}

/// A base [Tile] implementation to be used with a [TileMap] for rendering.
///
/// # Properties
///
/// * `glyph`: The symbol to use when rendering the [MapTile] on a [TileMap].
/// * `kind`: The [MapTileType] of the [MapTile]. Used to evaluate collision.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.8`
///
/// # See also
///
/// * [Tile]
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MapTile {
    /// The symbol to use when rendering the [MapTile] on a [TileMap].
    pub glyph: char,
    /// The [MapTileType] of the [MapTile]. Used to evaluate collision.
    pub kind: MapTileType,
}

impl MapTile {
    /// Creates a new [MapTile] of the [MapTileType::Floor] with the passed `glyph`.
    ///
    /// # Arguments
    ///
    /// * `glyph`: The symbol to use when rendering the [MapTile].
    ///
    /// returns: [MapTile]
    ///
    /// # Examples
    ///
    /// ```
    /// let map_tile = MapTile::floor('.');
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    /// # See also
    ///
    /// * [Tile]
    /// * [MapTileType]
    ///
    pub fn floor(glyph: char) -> Self {
        Self {
            glyph,
            kind: MapTileType::Floor,
        }
    }
}

impl Default for MapTile {
    fn default() -> Self {
        Self {
            glyph: '#',
            kind: MapTileType::Wall,
        }
    }
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.glyph, self.kind)
    }
}

impl Tile for MapTile {
    fn glyph(&self) -> char {
        self.glyph
    }

    fn foreground_color(&self, _is_seen: bool, is_visible: bool) -> Color {
        if is_visible {
            Color::SEA_GREEN
        } else {
            colors::INACTIVE
        }
    }

    fn background_color(&self, _is_seen: bool, _is_visible: bool) -> Color {
        colors::BACKGROUND
    }

    fn has_collision(&self) -> bool {
        self.kind == MapTileType::Wall
    }
}

/// Defines all possible kinds of [MapTile]s which can be rendered on a [TileMap].
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MapTileType {
    /// A standard, walkable tile, which makes up the default floor of the map.
    Floor,
    /// An impassable tile, marking the position it occupies as not walkable.
    /// Serves as the default barrier on the map.
    Wall,
}

impl Display for MapTileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MapTileType::Floor => write!(f, "Floor"),
            MapTileType::Wall => write!(f, "Wall"),
        }
    }
}
