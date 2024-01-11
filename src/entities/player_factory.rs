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

use bevy::prelude::{Color, Commands, Entity};

use crate::ascii_sprite;
use crate::components::coord_2d::Coord2d;
use crate::components::fov::Fov;
use crate::components::player::Player;
use crate::components::state_label::GameStateLabel;
use crate::core::position_2d::Position2d;
use crate::ui::colors;

/// Factory defining the markup of the `player` `entity` and handling its creation logic.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
pub struct PlayerFactory;

impl PlayerFactory {
    /// Creates and spawns a new player [Entity].
    ///
    /// # Arguments
    ///
    /// * `commands`: [Commands] queue required to spawn the player entity.
    /// * `starting_position`: The position of the `player entity` in the game world.
    ///
    /// returns: [Entity]
    ///
    /// # Examples
    ///
    /// ```
    /// fn spawner_system(mut commands: Commands) {
    ///     PlayerBundle::spawn(&mut commands, [40, 25]);
    /// }
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn spawn(commands: &mut Commands, starting_position: &impl Position2d) -> Entity {
        commands
            .spawn((
                Coord2d::from_position(starting_position),
                ascii_sprite!('@', Color::ORANGE, colors::BACKGROUND),
                Fov::new(8),
            ))
            .insert((Player, GameStateLabel))
            .id()
    }
}

impl Debug for PlayerFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ECS -> Entities -> PlayerFactory")
    }
}

impl Display for PlayerFactory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayerFactory")
    }
}
