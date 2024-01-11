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

use bevy::prelude::{Query, With, Without};
use bevy_ascii_terminal::Terminal;

use crate::components::ascii_sprite::AsciiSprite;
use crate::components::coord_2d::Coord2d;
use crate::ui::game_map::GameMap;
use crate::components::game_terminal::GameTerminal;
use crate::components::player::Player;
use crate::core::position_2d::Position2d;
use crate::ui::tile::Tile;
use crate::ui::tile_map::TileMap;

/// Renders the next frame of the game which includes the [GameMap] and all renderable
/// [AppState::Game] state relevant `entities`, e.g., monsters, items, etc.
///
/// # Arguments
///
/// * `terminal_query`: [Query] to retrieve the [Terminal], in order to render the next frame.
/// * `game_map_query`: [Query] to retrieve the [GameMap] for rendering.
/// * `player_query`: [Query] to retrieve the render data for the `player entity`.
/// * `actors_query`: [Query] to retrieve the render data for all other renderable `entities`.
///
/// # Panics
///
/// * If any of the set [Query] calls fail.
/// * If any of the required components can't be retrieved.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
/// # See also
/// * [Query]
/// * [Terminal]
/// * [GameMap]
/// * [Coord2d]
/// * [AsciiSprite]
///
pub fn render_system(
    mut terminal_query: Query<&mut Terminal, With<GameTerminal>>,
    game_map_query: Query<&GameMap>,
    player_query: Query<(&Coord2d, &AsciiSprite), With<Player>>,
    actors_query: Query<(&Coord2d, &AsciiSprite), Without<Player>>,
) {
    let mut terminal = terminal_query
        .get_single_mut()
        .expect("ECS -> Systems -> render_system -> Unable to retrieve {Terminal} component!");

    terminal.clear();

    let game_map = game_map_query
        .get_single()
        .expect("ECS -> Systems -> render_system -> Unable to retrieve {GameMap} component!");

    game_map.render(&mut terminal);

    for (coord, sprite) in actors_query.iter() {
        sprite.render(
            &coord.as_array(),
            &mut terminal,
            game_map.is_tile_seen(coord),
            game_map.is_tile_visible(coord),
        );
    }

    let (player_position, player_sprite) = player_query.get_single().expect(
        "ECS -> Systems -> render_system -> Unable to retrieve {Coord2d} and/or {AsciiSprite} component \
        for the player entity!"
    );

    player_sprite.render(player_position, &mut terminal, true, true);
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup, Update};
    use bevy_ascii_terminal::TerminalBundle;

    use crate::core::dimension_2d::Dimension2d;
    use crate::plugins::game_state_systems::lifecycle::startup_system;
    use crate::res::window_config::WindowConfig;

    use super::*;

    #[test]
    fn test_render_system() {
        let mut app = App::new();

        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, render_system);

        app.world
            .spawn(TerminalBundle::from(Terminal::new([100, 80])))
            .insert(GameTerminal);

        app.update();

        let game_map = app.world.query::<&GameMap>().single(&app.world);
        let center_coord = game_map.rooms().first().unwrap().center();

        assert_eq!(
            '@',
            app.world
                .query::<&Terminal>()
                .single(&app.world)
                .get_char(center_coord)
        )
    }
}
