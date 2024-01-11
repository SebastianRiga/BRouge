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

use crate::core::constants;
use crate::core::rng::RandomNumberGenerator;
use crate::ui::game_map::GameMap;
use crate::ui::rectangle::Rectangle;

pub trait TileMapLayoutGenerator {
    fn generate_layout(&self, map: &mut GameMap);
}

pub struct BaseTileMapGenerator;

impl TileMapLayoutGenerator for BaseTileMapGenerator {
    fn generate_layout(&self, map: &mut GameMap) {
        let mut rng = RandomNumberGenerator::new();

        'rooms: for _ in 0..constants::MAP_MAX_ROOMS {
            let room_width = rng.range(constants::MAP_MIN_ROOM_SIZE..=constants::MAP_MAX_ROOM_SIZE);

            let room_height =
                rng.range(constants::MAP_MIN_ROOM_SIZE..=constants::MAP_MAX_ROOM_SIZE);

            let room = Rectangle::new(
                [
                    rng.roll_dice(1, map.width - room_width - 1) - 1,
                    rng.roll_dice(1, map.height - room_height - 1) - 1,
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
                room.connect(&previous_room, map);
            }

            room.add_to_map(map);
            map.rooms.push(room);
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::core::dimension_2d::Dimension2d;
    use crate::ui::game_map::GameMap;
    use crate::ui::tile::MapTile;
    use crate::ui::tile_map::TileMap;
    use crate::ui::tile_map_layout_generator::TileMapLayoutGenerator;

    pub struct TestTileMapGenerator;

    impl TileMapLayoutGenerator for TestTileMapGenerator {
        fn generate_layout(&self, map: &mut GameMap) {
            map.set_tile_at(&map.center(), MapTile::floor('.'));
        }
    }
}
