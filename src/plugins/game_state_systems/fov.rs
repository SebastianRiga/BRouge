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

use bevy::prelude::{Mut, Query, With, Without};

use crate::components::coord_2d::Coord2d;
use crate::components::fov::Fov;
use crate::ui::game_map::GameMap;
use crate::components::player::Player;
use crate::core::algorithm::field_of_view;
use crate::ui::tile_map::TileMap;

/// System to calculate and update the [Fov] of `actor` `entities` such as the `player`,
/// `monsters`, `NPC`s, etc., while the `player` traverses the game's world.
///
/// # Arguments
///
/// * `game_map_query`: [Query] required to retrieve the game map for the
/// `field of view` calculation.
/// * `fov_query`: [Query] required to retrieve and update the `field of view`
/// of all `non-player entities`.
/// * `player_fov_query`: [Query] required to retrieve and update the `field of view`
/// of the `player entity`.
///
/// returns: ()
///
/// # Panics
///
/// * If any of the [Query] calls fail.
/// * If any of the required components can't be retrieved from the ECS.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
pub fn fov_system(
    mut game_map_query: Query<&mut GameMap>,
    mut fov_query: Query<(&mut Fov, &Coord2d), Without<Player>>,
    mut player_fov_query: Query<(&mut Fov, &Coord2d), With<Player>>,
) {
    let map = game_map_query
        .get_single_mut()
        .expect("ECS -> Systems -> fov_system -> Unable to retrieve {GameMap} component!")
        .into_inner();

    for (mut fov, position) in fov_query.iter_mut() {
        field_of_view(&mut fov, position, map);
    }

    // Calculate `field of view` for the `player entity`.
    let (mut player_fov, player_position): (Mut<Fov>, &Coord2d) = player_fov_query.get_single_mut().expect(
        "ECS -> Systems -> fov_system -> Unable to retrieve the player's {Fov} and/or {Coord2d} components!"
    );

    field_of_view(&mut player_fov, player_position, map);

    // Update the `GameMap` with the `field of view` calculation result of the `player entity`.
    map.reset_visible_tiles();

    for position in player_fov.positions() {
        map.mark_tile_as_seen(position);
        map.mark_tile_as_visible(position);
    }
}
