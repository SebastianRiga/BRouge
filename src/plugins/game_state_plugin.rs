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

use bevy::app::{App, Plugin, PostUpdate};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Update};

use crate::plugins::game_state_systems::{enemy_ai, fov, graphics, input, lifecycle};
use crate::plugins::states::AppState;

/// Plugin coupled with the [AppState::Game] state, which makes up the main gameplay state.
/// In it the user moves the `player entity`, fights or otherwise interacts with the game.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
/// # See also
///
/// * [Plugin]
/// * [AppState::Game]
///
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), lifecycle::startup_system)
            .add_systems(
                Update,
                (
                    input::keyboard_input_system,
                    fov::fov_system,
                    graphics::render_system,
                    enemy_ai::enemy_line_of_sight_system,
                )
                    .chain()
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                PostUpdate,
                lifecycle::npc_turn_end_system.run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), lifecycle::shutdown_system);
    }

    fn name(&self) -> &str {
        "ECS -> Plugins -> GameState"
    }

    fn is_unique(&self) -> bool {
        true
    }
}
