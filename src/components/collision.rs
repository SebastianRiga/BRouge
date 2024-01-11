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

/// Marker interface denoting the associated entity as having collision, i.e. the player can't
/// move onto the space occupied by the given entity.
///
/// # Examples
///
/// ```
/// let entity = commands.spawn((
///     Coord2d::from_position(position),
///     ascii_sprite!('m', Color::YELLOW),
///     Fov::new(8),
///     NameTag::new("Mended"),
///     EnemyType::Mended,
///     NpcState::default(),
///     Collision, // The spawned `entity` will block the space it occupies in the game's world.
/// )).insert(GameStateLabel);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.9`
///
#[derive(Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct Collision;

impl Debug for Collision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ECS -> Components -> Collision {{ (Marker) }}")
    }
}

impl Display for Collision {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Marker(Collision)")
    }
}
