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

use bevy::prelude::States;

/// Defines all states the game can be in, with every state representing an isolated and distinct logic section
/// in the game's state machine.
///
/// Every state has governance over its own entities, [bevy::prelude::Component]s, [bevy::prelude::Resource]s,
/// and logic, e.g., one state might display the title screen with a specific main menu UI and music, while another
/// state handle the main gameplay logic of the game.
///
/// To facilitate this logic and separation, every [AppState] is coupled tightly with a respective
/// [bevy::prelude::Plugin], which defines the states logic. The corresponding [bevy::prelude::Plugin] is only active
/// when the state machine is in the coupled state.
///
/// # Examples
///
/// See the [crate::plugins::game_state_plugin::GameStatePlugin] for an example implementation.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    /// The main gameplay state, in which the player actively travers the world and interacts with the game.
    ///
    /// See the [crate::plugins::game_state_plugin::GameStatePlugin] for the corresponding [bevy::prelude::Plugin].
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    #[default]
    Game,
}
