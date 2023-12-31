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

use bevy::prelude::Mut;
use bevy_ascii_terminal::Terminal;

use crate::core::dimension_2d::Dimension2d;
use crate::core::position_2d::Position2d;
use crate::ui::tile::Tile;

/// A map of [Tile]s, which can be rendered on demand. While the map groups the [Tile]s and initiates their rendering
/// through delegation, the [Tile]s are responsible for their visual representation.
///
/// A [TileMap] makes up a specific section of arbitrary size on the screen, e.g.,
/// * the in-game map
/// * message boxes
/// * menus
/// * dialogs
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
/// * [Tile]
///
pub trait TileMap<T: Tile>: Dimension2d {
    /// Converts the passed `index` to its respective `usize` position in the world space.
    ///
    /// # Parameters
    ///
    /// * `T`: The [Tile] implementation the [TileMap] can display.
    ///
    /// # Arguments
    ///
    /// * `width`: The width of the [TileMap], required for the `index` conversion.
    /// * `index`: The [Tile] index in the [TileMap] space to convert.
    ///
    /// returns: usize
    ///
    /// # Examples
    ///
    /// ```
    /// // Internally the game map uses a linear vector to store the tiles of size `width * height`, e.g., 80 * 50.
    /// let map = GameMap::new([80, 50]);
    ///
    /// // Internally the passed position is converted to index `3225` in the linear index.
    /// map.get_tile_at([40, 25]);
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
    /// * [Terminal]
    /// * [Tile]
    /// * [Position2d]
    ///
    fn convert_world_index(width: i32, index: &impl Position2d) -> usize {
        (index.y_coordinate() as usize * width as usize) + index.x_coordinate() as usize
    }

    /// Returns an immutable reference to [Tile]s of the map.
    ///
    /// # Arguments
    ///
    /// returns: &[Vec]`<T>`
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
    fn tiles(&self) -> &Vec<T>;

    /// Returns a mutable reference to [Tile]s of the map.
    ///
    /// # Arguments
    ///
    /// returns: &mut [Vec]`<T>`
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
    fn tiles_mut(&mut self) -> &mut Vec<T>;

    /// Returns an immutable reference to the [Tile] at the given `index`.
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based index of the [Tile] to query.
    ///
    /// returns: &T
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    /// # See also
    ///
    /// * [Position2d]
    /// * [Tiled]
    ///
    fn get_tile_at(&self, index: &impl Position2d) -> &T {
        &self.tiles()[Self::convert_world_index(self.width(), index)]
    }

    /// Sets the passed `tile` at the given `index` on the [TileMap].
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based index at which the `tile` should be set.
    /// * `tile`: The [Tile] to set at the given `index`.
    ///
    /// returns: ()
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    /// # See also
    ///
    /// * [Position2d]
    /// * [Tile]
    ///
    fn set_tile_at(&mut self, index: &impl Position2d, tile: T) {
        let width = self.width();
        self.tiles_mut()[Self::convert_world_index(width, index)] = tile;
    }

    /// Checks if the [Tile] at the passed `index` has collision.
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based index of the [Tile] to check.
    ///
    /// returns: bool - `true` if the [Tile] has collision and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// fn movement_system(player_query: Query<&Coord2d, With<Player>, game_map_query: Query<&GameMap>) {
    ///     if let Ok(player_position) = player_query.get_single() {
    ///         if let Ok(game_map) = game_map_query.get_single() {
    ///             if game_map.has_collision(player_position) {
    ///                 // Update player position.
    ///             } else {
    ///                 // Unable to move, execute other action...
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
    /// Since: `0.1.8`
    ///
    /// # See also
    ///
    /// * [Tile]
    /// * [Position2d]
    ///
    fn tile_has_collision(&self, index: &impl Position2d) -> bool;

    /// Checks if the [Tile] at the given `index` has been seen by the `player` at any point during gameplay.
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based index of the tile to check.
    ///
    /// returns: bool - `true` if the [Tile] has been seen by the `player` and `false` otherwise.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    /// # See also
    ///
    /// * [Position2d]
    ///
    fn is_tile_seen(&self, index: &impl Position2d) -> bool;

    /// Marks the [Tile] at the passed `index` as seen, i.e. it was in the `player`s `field of view` at some point
    /// during gameplay.
    ///
    /// [Tile]s which are marked as seen, are usually not invisible, even when they are no longer in the `player`s
    /// current `field of view`.
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based [Tile] index to mark as seen.
    ///
    /// returns: ()
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
    /// * [Position2d]
    ///
    fn mark_tile_as_seen(&mut self, index: &impl Position2d);

    /// Checks if the [Tile] at the given `index` is currently visible, i.e. in the `field of view` of the `player`.
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based index of the tile to check.
    ///
    /// returns: bool - `true` if the [Tile] is in the `field of view` of the `player` and `false` otherwise.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    /// # See also
    ///
    /// * [Position2d]
    ///
    fn is_tile_visible(&self, index: &impl Position2d) -> bool;

    /// Marks the [Tile] at the passed `index` as visible, i.e. it is in the `player`s current `field of view`.
    ///
    /// [Tile]s in the `field of view` of the `player` are usually rendered differently, to emphasize the part of
    /// the [TileMap] as the active play area.
    ///
    /// # Arguments
    ///
    /// * `index`: The [Position2d] based [Tile] position to mark as visible.
    ///
    /// returns: ()
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
    /// * [Position2d]
    ///
    fn mark_tile_as_visible(&mut self, index: &impl Position2d);

    /// Removes all currently visible [Tile]s from the [TileMap]s respective backing storage.
    ///
    /// The type of storage the [TileMap] uses, e.g. an `array`, a [Vec] variant, are up to the implementation.
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
    fn reset_visible_tiles(&mut self);

    /// Renders all tiles which make up the map on screen on the passed [Terminal].
    ///
    /// # Arguments
    ///
    /// * `terminal`: [Terminal] which handles the actual rendering.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// struct Map<TileImpl> {
    ///     pub tiles: vec1[TileImpl::default(); 80]
    /// }
    ///
    /// impl TileMap for Map {
    ///     fn render(&self, terminal: &mut Mut<Terminal>) {
    ///         for x in 0..80 {
    ///             self.tiles[0].render_at([x, 1], terminal);
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
    /// * [Tile]
    ///
    fn render(&self, terminal: &mut Mut<Terminal>) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                let position_2d = [x, y];
                let index = Self::convert_world_index(self.width(), &position_2d);

                self.tiles()[index].render(
                    &position_2d,
                    terminal,
                    self.is_tile_seen(&position_2d),
                    self.is_tile_visible(&position_2d),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {}
