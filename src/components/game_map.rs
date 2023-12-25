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

use crate::core::constants;
use crate::core::dimension_2d::Dimension2d;
use crate::core::fov_algorithm::PlayArea2d;
use crate::core::position_2d::Position2d;
use crate::core::rectangle::Rectangle;
use crate::core::rng::RandomNumberGenerator;
use crate::core::tile::TileType;
use crate::core::var_args::VarArgs;
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
/// * `seen_tiles`: (Private) List of all tiles which the player has seen before,
/// e.g., which were in his FOV at least once.
/// * `visible_tiles`: (Private) List of all tiles which the player currently sees, as defined by their FOV.
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
    /// List of all rooms on the map in form of [Rectangle]s.
    pub rooms: Vec<Rectangle>,
    /// (Private) List of all tiles which make up the map as a linear vector.
    tiles: Vec<TileType>,
    /// (Private) List of all tiles which the player has seen before, e.g., which were in his FOV at least once.
    seen_tiles: Vec<bool>,
    /// (Private) List of all tiles which the player currently sees, as defined by their FOV.
    visible_tiles: Vec<bool>,
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
        let width = dimension.width();
        let height = dimension.height();
        let area = dimension.area();

        let mut map = Self {
            width,
            height,
            rooms: Vec::new(),
            tiles: vec![TileType::Wall; area],
            seen_tiles: vec![false; area],
            visible_tiles: vec![false; area],
        };

        let mut rng = RandomNumberGenerator::new();

        'rooms: for _ in 0..constants::MAP_MAX_ROOMS {
            let room_width = rng.range(constants::MAP_MIN_ROOM_SIZE..=constants::MAP_MAX_ROOM_SIZE);
            let room_height =
                rng.range(constants::MAP_MIN_ROOM_SIZE..=constants::MAP_MAX_ROOM_SIZE);

            let room = Rectangle::new(
                [
                    rng.roll_dice(1, width - room_width - 1) - 1,
                    rng.roll_dice(1, height - room_height - 1) - 1,
                ],
                [room_width, room_height],
            );

            for existing_room in map.rooms.iter() {
                if room.collides(existing_room) {
                    continue 'rooms;
                }
            }

            if !map.rooms.is_empty() {
                let previous_room = map.rooms[map.rooms.len() - 1];
                room.connect(&previous_room, &mut map);
            }

            room.add_to_map(&mut map);
        }

        map
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `index`:
    /// * `tile_type`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn set_tile_at(&mut self, index: &impl Position2d, tile_type: TileType) {
        self.tiles[Self::convert_world_index(self.width, index)] = tile_type;
    }

    ///
    ///
    /// # Arguments
    ///
    /// * `width`: The width of the map
    /// * `index`:
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
    /// Since: `0.1.7`
    ///
    fn convert_world_index(width: i32, index: &impl Position2d) -> usize {
        (index.y_coordinate() as usize * width as usize) + index.x_coordinate() as usize
    }
}

impl ViewGroup for GameMap {
    fn render(&self, terminal: &mut Mut<Terminal>) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = Self::convert_world_index(self.width, &[x, y]);
                self.tiles[index].render_at(
                    &[x, y],
                    terminal,
                    VarArgs::new()
                        .insert("visible", self.visible_tiles[index])
                        .insert("seen", self.seen_tiles[index]),
                );
            }
        }
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

impl PlayArea2d for GameMap {
    fn collides(&self, index: &impl Position2d) -> bool {
        self.tiles[Self::convert_world_index(self.width, index)] == TileType::Wall
    }

    fn mark_tile_as_seen(&mut self, index: &impl Position2d) {
        self.seen_tiles[Self::convert_world_index(self.width, index)] = true
    }

    fn mark_tile_as_visible(&mut self, index: &impl Position2d) {
        self.visible_tiles[Self::convert_world_index(self.width, index)] = true
    }

    fn clear_visible_tiles(&mut self) {
        self.visible_tiles.clear();
        self.visible_tiles.resize(self.area(), false);
    }
}
