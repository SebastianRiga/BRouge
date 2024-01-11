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

use bevy::log::info;
use bevy::prelude::{Query, Res, With};

use crate::components::coord_2d::Coord2d;
use crate::components::enemy_type::EnemyType;
use crate::components::fov::Fov;
use crate::components::name_tag::NameTag;
use crate::components::npc_state::NpcState;
use crate::components::player::Player;
use crate::plugins::states::GameTurnState;

/// Computes the respective enemy's reaction to the `player` entering or being inside their `field of view`.
///
/// This system is only executed if the game's [GameTurnState] matches [GameTurnState::Npc].
///
/// # Arguments
///
/// * `game_turn_state`: The [GameTurnState] resource required to verify that it's the enemy's turn.
/// * `enemy_fov_query`: [Query] required to retrieve the [Fov] components of the respective enemies.
/// * `player_position_query`: [Query] to retrieve the `player entities` position.
///
/// returns: ()
///
/// # Panics
///
/// * If any of the [Query] calls fail.
/// * If any of the required components can't be retrieved from the ECS.
/// * If any of the required resources can't be retrieved from the ECS.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.9`
///
pub fn enemy_line_of_sight_system(
    game_turn_state: Res<GameTurnState>,
    mut enemy_query: Query<(&Fov, &NameTag, &EnemyType, &mut NpcState)>,
    player_position_query: Query<&Coord2d, With<Player>>,
) {
    if game_turn_state.into_inner() != &GameTurnState::Npc {
        return;
    }

    let player_position = player_position_query.get_single().expect(
        "ECS -> Systems -> enemy_view_contact_system -> Unable to retrieve the player's {Coord2d} component!",
    );

    for (fov, name_tag, enemy_type, mut npc_state) in enemy_query.iter_mut() {
        if fov.contains(player_position) {
            if npc_state.is_seeing_player {
                return;
            }

            npc_state.is_seeing_player = true;

            match enemy_type {
                EnemyType::Mended => info!("{} gurgles and shifts at your presence.", name_tag),
            }
        } else {
            npc_state.is_seeing_player = false;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_enemy_line_of_sight_system() {}
}
