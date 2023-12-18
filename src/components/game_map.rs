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

use bevy::prelude::{Component, Mut};
use bevy_ascii_terminal::Terminal;

use crate::core::dimension_2d::Dimension2d;
use crate::core::position_2d::Position2d;
use crate::core::tile::TileType;
use crate::core::view::{View, ViewGroup};

/// A map making up a level of the game, which the `player` can traverse and explore.
///
/// It is made up of a linear vector of tiles in which the different `entities` of the reside in.
///
/// # Properties
///
/// * `width`: The real width of the map.
/// * `height`: The real height of the map.
/// * `tiles`: (Private) List of all tiles which make up the map as a linear vector.
///
/// # Examples
///
/// ```
/// fn startup_system(mut commands: Commands) {
///    commands.spawn(GameMap::new([80, 50]));
/// }
///
/// fn render_system(mut terminal_query: Query<&mut Terminal>, game_map_query: Query<&GameMap>) {
///     let mut terminal = terminal_query.single_mut();
///
///     terminal.clear();
///
///     game_map_query.single().render(&mut terminal);
/// }
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
#[derive(Debug, Clone, Component)]
pub struct GameMap {
    /// The real width of the map.
    pub width: i32,
    /// The real height of the map.
    pub height: i32,
    /// (Private) List of all tiles which make up the map as a linear vector.
    tiles: Vec<TileType>,
}

impl GameMap {
    /// Creates a new [GameMap] instance with the passed `dimension`.
    ///
    /// # Arguments 
    ///
    /// * `dimension`: The [Dimension2d] with which the map should be created.
    ///
    /// returns: [GameMap] 
    ///
    /// # Examples 
    ///
    /// ```
    /// fn startup_system(mut commands: Commands) {
    ///    commands.spawn(GameMap::new([80, 50]));
    /// }
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn new(dimension: impl Dimension2d) -> Self {
        Self {
            width: dimension.width(),
            height: dimension.height(),
            tiles: vec![TileType::Floor; dimension.area()],
        }
    }

    /// Retrieves the tile at the passed `index` as a [TileType] reference.
    ///
    /// # Arguments
    ///
    /// * `index`: The index at which the [TileType] should be retrieved.
    ///
    /// returns: &[TileType]
    ///
    /// # Examples
    ///
    /// ```
    /// let game_map = GameMap::new([80, 50]);
    ///
    /// let tile = game_map.get_tile_at([40, 25]);
    ///
    /// match tile {
    ///     ...
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// If the passed `index` is out of bounds of the map.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn get_tile_at(&self, index: impl Position2d) -> &TileType {
        &self.tiles[(index.y() as usize * self.width as usize) + index.x() as usize]
    }
}

impl ViewGroup for GameMap {
    fn render(&self, terminal: &mut Mut<Terminal>) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.get_tile_at([x, y]).render_at([x, y], terminal);
            }
        }
    }
}
