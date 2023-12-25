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

use bevy::prelude::{Bundle, Color, Commands, Entity};

use crate::ascii_sprite;
use crate::components::ascii_sprite::AsciiSprite;
use crate::components::coord_2d::Coord2d;
use crate::components::fov::Fov;
use crate::components::player::Player;
use crate::core::position_2d::Position2d;

/// [Bundle] defining the markup of the `player` `entity` and handling its spawning logic.
///
/// # Properties
///
/// * `player`: Marker [bevy::prelude::Component] used to identify respective entity
/// as the player / main actor of the game.
/// * `position`: The starting position of the player entity in the game's world
/// when being spawned.
/// * `sprite`: Renderable sprite representing the player entity on-screen.
/// * `fov`: The radius around the player in which tiles, monsters and items are visible.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    /// Marker [bevy::prelude::Component] used to identify respective entity
    /// as the player / main actor of the game.
    ///
    /// The resulting entity will be controllable by the player,
    /// fight monsters, end the game when it dies, etc.
    pub player: Player,
    /// The starting position of the player entity in the game's world when being spawned.
    pub position: Coord2d,
    /// Renderable sprite representing the `player` `entity` on-screen.
    pub sprite: AsciiSprite,
    /// The radius around the player in which tiles, monsters and items are visible.
    pub fov: Fov,
}

impl PlayerBundle {
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
            .spawn(Self {
                player: Player,
                position: Coord2d::from_position(starting_position),
                sprite: ascii_sprite!('@', Color::ORANGE, Color::BLACK),
                fov: Fov::new(8),
            })
            .id()
    }
}
