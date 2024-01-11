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

use crate::core::dimension_2d::Dimension2d;
use crate::core::position_2d::Position2d;
use crate::ui::rectangle::Rectangle;
use crate::ui::tile::{MapTile, Tile};
use crate::ui::tile_map::TileMap;
use crate::ui::tile_map_layout_generator::TileMapLayoutGenerator;

/// A map making up a level of the game, which the `player` can traverse and explore.
///
/// It is made up of a linear vector of tiles in which the different `entities` of the reside in.
///
/// # Properties
///
/// * `width`: The real width of the map.
/// * `height`: The real height of the map.
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
#[derive(Clone, Component)]
pub struct GameMap {
    /// The real width of the map.
    pub width: i32,
    /// The real height of the map.
    pub height: i32,
    /// (Package-Private) List of all rooms on the map in form of [Rectangle]s.
    pub(super) rooms: Vec<Rectangle>,
    /// (Package-Private) List of all tiles which make up the map as a linear vector.
    pub(super) tiles: Vec<MapTile>,
    /// (Package-Private) List of all tiles which the player has seen before, e.g., which were in his FOV at least once.
    pub(super) seen_tiles: Vec<bool>,
    /// (Package-Private) List of all tiles which the player currently sees, as defined by their FOV.
    pub(super) visible_tiles: Vec<bool>,
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
    pub fn new(dimension: &impl Dimension2d, generator: &impl TileMapLayoutGenerator) -> Self {
        let width = dimension.width();
        let height = dimension.height();
        let area = dimension.area();

        let mut map = Self {
            width,
            height,
            rooms: Vec::new(),
            tiles: vec![MapTile::default(); area],
            seen_tiles: vec![false; area],
            visible_tiles: vec![false; area],
        };

        generator.generate_layout(&mut map);

        map
    }

    /// Returns an immutable [Vec] reference containing all the rooms on the map as [Rectangle] instances.
    ///
    /// # Arguments
    ///
    /// returns: &[Vec]<[Rectangle]>
    ///
    /// # Examples
    ///
    /// ```
    /// let map = GameMap::new([80, 50]);
    ///
    /// ...
    ///
    /// for room in map.rooms().iter() {
    ///     // Use the room
    /// }
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.8`
    ///
    pub fn rooms(&self) -> &Vec<Rectangle> {
        &self.rooms
    }
}

impl Debug for GameMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ECS -> Components -> GameMap {{\
            width: {:?}, \
            height: {:?}, \
            rooms: {:?}, \
            tiles: {:?}, \
            seen_tiles: {:?}, \
            visible_tiles: {:?}\
            }}",
            self.width, self.height, self.rooms, self.tiles, self.seen_tiles, self.visible_tiles
        )
    }
}

impl Display for GameMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}, {}, {}, {})",
            self.width,
            self.height,
            self.rooms.len(),
            self.tiles.len(),
            self.seen_tiles.len(),
            self.visible_tiles.len()
        )
    }
}

impl Dimension2d for GameMap {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }
}

impl TileMap<MapTile> for GameMap {
    fn tiles(&self) -> &Vec<MapTile> {
        &self.tiles
    }

    fn tiles_mut(&mut self) -> &mut Vec<MapTile> {
        &mut self.tiles
    }

    fn tile_has_collision(&self, index: &impl Position2d) -> bool {
        self.get_tile_at(index).has_collision()
    }

    fn is_tile_seen(&self, index: &impl Position2d) -> bool {
        self.seen_tiles[Self::convert_world_index(self.width, index)]
    }

    fn mark_tile_as_seen(&mut self, index: &impl Position2d) {
        self.seen_tiles[Self::convert_world_index(self.width, index)] = true
    }

    fn is_tile_visible(&self, index: &impl Position2d) -> bool {
        self.visible_tiles[Self::convert_world_index(self.width, index)]
    }

    fn mark_tile_as_visible(&mut self, index: &impl Position2d) {
        self.visible_tiles[Self::convert_world_index(self.width, index)] = true
    }

    fn reset_visible_tiles(&mut self) {
        self.visible_tiles.clear();
        self.visible_tiles.resize(self.area(), false);
    }
}
