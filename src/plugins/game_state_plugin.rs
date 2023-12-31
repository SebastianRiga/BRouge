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

use bevy::app::{App, AppExit, Plugin};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::log::info;
use bevy::prelude::{
    in_state, Commands, Entity, EventReader, EventWriter, IntoSystemConfigs, Mut, OnEnter, OnExit,
    Query, Res, Update, With,
};
use bevy_ascii_terminal::Terminal;

use crate::components::ascii_sprite::AsciiSprite;
use crate::components::coord_2d::Coord2d;
use crate::components::fov::Fov;
use crate::components::game_map::GameMap;
use crate::components::game_terminal::GameTerminal;
use crate::components::player::Player;
use crate::core::dimension_2d::Dimension2d;
use crate::core::fov_algorithm::field_of_view;
use crate::core::position_2d::Position2d;
use crate::entities::player_factory::PlayerFactory;
use crate::plugins::app_state::AppState;
use crate::res::input_config::{InputConfig, InputType};
use crate::res::window_config::WindowConfig;
use crate::ui::tile::Tile;
use crate::ui::tile_map::TileMap;

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
        app.add_systems(OnEnter(AppState::Game), startup_system)
            .add_systems(
                Update,
                (input_system, fov_system, render_system).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnExit(AppState::Game), shutdown_system);
    }

    fn name(&self) -> &str {
        "BRouge: Game state"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// System which is run when the game's state machine changes into the [AppState::Game] state to spawn all required
/// `entities`.
///
/// # Arguments
///
/// * `commands`: [Commands] queue required to spawn the necessary `entities`.
/// * `window_config`: [WindowConfig] resource required to check the bounds of the game's window during the
/// `entity` creation.
///
/// returns: ()
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
fn startup_system(mut commands: Commands, window_config: Res<WindowConfig>) {
    let game_map = GameMap::new(window_config.terminal_size());
    let starting_position = game_map.rooms().first().unwrap().center();

    commands.spawn(game_map);
    PlayerFactory::spawn(&mut commands, &starting_position);
}

/// System to handle user's input through any of the connected peripherals like the keyboard or mouse.
///
/// # Arguments
///
/// * `input_config`: [InputConfig] required to recognize the user's input.
/// * `window_config`: [WindowConfig] required to check the bounds of the game's map when moving the player.
/// * `key_events`: [EventReader] stream of [KeyboardInput] events required to parse the user's input.
/// * `player_position_query`: [Query] to retrieve the position of the `player entity`, required to move him according
/// to the user's input (if applicable).
/// * `exit_event`: [EventWriter] to send the [AppExit] event to the game's engine in order to close the game.
///
/// returns: ()
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
fn input_system(
    input_config: Res<InputConfig>,
    game_map_query: Query<&GameMap>,
    mut key_events: EventReader<KeyboardInput>,
    mut player_query: Query<(&mut Fov, &mut Coord2d), With<Player>>,
    mut exit_event: EventWriter<AppExit>,
) {
    for event in key_events.read() {
        if event.state == ButtonState::Released || event.key_code.is_none() {
            return;
        }

        if let Some(input) = input_config.parse_input(event.key_code.unwrap()) {
            info!("Received keyboard input event: {:?}", input);

            let (mut fov, mut position) = player_query.single_mut();

            if input.is_movement_event() {
                handle_player_movement(game_map_query.single(), &input, &mut fov, &mut position)
            }

            if input == InputType::Cancel {
                exit_event.send(AppExit)
            }
        }
    }
}

/// System to calculate and update the `player`'s `field of view`, as the `player` traverses the game's world.
///
/// # Arguments
///
/// * `game_map_query`: [Query] required to retrieve the game map for the `field of view` calculation.
/// * `fov_query`: [Query] required to retrieve and update the `player`'s `field of view` component.
///
/// returns: ()
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
fn fov_system(
    mut game_map_query: Query<&mut GameMap>,
    mut fov_query: Query<(&mut Fov, &Coord2d), With<Player>>,
) {
    let mut map = game_map_query.single_mut();
    let (mut fov, position) = fov_query.single_mut();
    field_of_view(&mut fov, position, &mut map)
}

/// Renders the next frame of the game which includes the [GameMap] and all renderable [AppState::Game] state relevant
/// `entities`, e.g., monsters, items, etc.
///
/// # Arguments
///
/// * `terminal_query`: [Query] to retrieve the [Terminal], in order to render the next frame.
/// * `game_map_query`: [Query] to retrieve the [GameMap] for rendering.
/// * `render_query`: [Query] to retrieve all renderable entities.
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
fn render_system(
    mut terminal_query: Query<&mut Terminal, With<GameTerminal>>,
    game_map_query: Query<&GameMap>,
    player_query: Query<(&mut Coord2d, &mut AsciiSprite), With<Player>>,
) {
    let mut terminal = terminal_query.single_mut();

    terminal.clear();

    game_map_query.single().render(&mut terminal);

    for (coord, sprite) in player_query.iter() {
        sprite.render(&coord.as_array(), &mut terminal, true, true);
    }
}

/// Clean up system, which is run when the game's state machine is leaving the [AppState::Game] state.
///
/// Removes all no longer required state specific `entities`, [bevy::prelude::Component]s
/// and [bevy::prelude::Resource]s.
///
/// # Arguments
///
/// * `commands`: [Commands] queue required to execute the associated operations in the ecs.
/// * `player_query`: [Query] to fetch the [Player] entity.
/// * `game_map_query`: [Query] to fetch the [GameMap] component.
///
/// returns: ()
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
///
fn shutdown_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    game_map_query: Query<Entity, With<GameMap>>,
) {
    commands
        .get_entity(player_query.single())
        .unwrap()
        .despawn();
    commands
        .get_entity(game_map_query.single())
        .unwrap()
        .despawn();
}

/// Internal function to update the `player entities` positional component according to the passed `input_type`
/// within the set `player_area`.
///
/// # Arguments
///
/// * `input_type`: The movement [InputType] according to which the `player_position` will be manipulated.
/// * `player_area`: The area in which the player can move.
/// * `player_position`: The [Coord2d] ecs [bevy::prelude::Component] of the `player` `entity`.
///
/// returns: ()
///
/// # Examples
///
/// ```
/// let mut player_position = Coord2d::new(40, 25);
/// handle_player_movement(InputType::Up, [80, 50]);
///
/// assert_eq!([40, 26], player_position.to_array());
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
/// # See also
///
/// * [InputType]
/// * [Dimension2d]
/// * [Coord2d]
///
fn handle_player_movement(
    game_map: &GameMap,
    input_type: &InputType,
    player_fov: &mut Mut<Fov>,
    player_position: &mut Mut<Coord2d>,
) {
    let new_position = match input_type {
        InputType::Up => player_position.up(game_map.height - 1),
        InputType::Left => player_position.left(0),
        InputType::Right => player_position.right(game_map.width - 1),
        InputType::Down => player_position.down(0),
        _ => Coord2d::from_position(&[player_position.x, player_position.y]),
    };

    if game_map.tile_has_collision(&new_position) {
        return;
    }

    if new_position != **player_position {
        player_fov.is_dirty = true;
        player_position.x = new_position.x;
        player_position.y = new_position.y;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_ascii_terminal::TerminalBundle;

    use super::*;

    #[derive(Component)]
    struct DummyComponent;

    #[test]
    fn test_startup_system() {
        let mut app = App::new();

        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.add_systems(Startup, startup_system);

        app.update();

        assert_eq!(1, app.world.query::<&Player>().iter(&app.world).len());

        assert_eq!(
            &AsciiSprite::new('@', Color::ORANGE, Color::BLACK),
            app.world.query::<&AsciiSprite>().single(&app.world)
        );

        assert_eq!(
            app.world.query::<&Coord2d>().single(&app.world).as_array(),
            app.world
                .query::<&GameMap>()
                .single(&app.world)
                .rooms()
                .first()
                .unwrap()
                .center()
        );

        assert_eq!(
            [100, 80],
            app.world.query::<&GameMap>().single(&app.world).as_array()
        );
    }

    #[test]
    fn test_input_system() {
        let mut app = App::new();

        app.add_event::<KeyboardInput>();
        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.insert_resource(InputConfig {
            up: KeyCode::W,
            left: KeyCode::A,
            down: KeyCode::S,
            right: KeyCode::D,
            cancel: KeyCode::Escape,
        });
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, input_system);

        // Test keyboard up press and resulting player movement

        let window = app.world.spawn(DummyComponent).id();

        #[allow(unused_assignments)]
        let mut player_coord: Coord2d = Coord2d::new(0, 0);

        app.update();

        {
            player_coord = *app
                .world
                .query_filtered::<&Coord2d, With<Player>>()
                .single(&app.world);
        }

        app.world.send_event(KeyboardInput {
            scan_code: 32,
            key_code: Some(KeyCode::W),
            state: ButtonState::Pressed,
            window,
        });

        app.update();

        player_coord = player_coord.up(640);

        assert_eq!(
            &player_coord,
            app.world
                .query::<(&Coord2d, With<Player>)>()
                .single(&app.world)
                .0
        );

        // Test keyboard left press and resulting player movement

        app.world.send_event(KeyboardInput {
            scan_code: 32,
            key_code: Some(KeyCode::A),
            state: ButtonState::Pressed,
            window,
        });

        app.update();

        player_coord = player_coord.left(0);

        assert_eq!(
            &player_coord,
            app.world
                .query::<(&Coord2d, With<Player>)>()
                .single(&app.world)
                .0
        );

        // Test keyboard down press and resulting player movement

        app.world.send_event(KeyboardInput {
            scan_code: 32,
            key_code: Some(KeyCode::S),
            state: ButtonState::Pressed,
            window,
        });

        app.update();

        player_coord = player_coord.down(0);

        assert_eq!(
            &player_coord,
            app.world
                .query::<(&Coord2d, With<Player>)>()
                .single(&app.world)
                .0
        );

        // Test keyboard right press and resulting player movement

        app.world.send_event(KeyboardInput {
            scan_code: 32,
            key_code: Some(KeyCode::D),
            state: ButtonState::Pressed,
            window,
        });

        app.update();

        player_coord = player_coord.right(800);

        assert_eq!(
            &player_coord,
            app.world
                .query::<(&Coord2d, With<Player>)>()
                .single(&app.world)
                .0
        );

        // Test unrecognized keyboard press and resulting player movement

        app.world.send_event(KeyboardInput {
            scan_code: 32,
            key_code: Some(KeyCode::Escape),
            state: ButtonState::Pressed,
            window,
        });

        app.update();

        assert_eq!(
            &player_coord,
            app.world
                .query::<(&Coord2d, With<Player>)>()
                .single(&app.world)
                .0
        );
    }

    #[test]
    fn test_fov_system() {}

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

    #[test]
    fn test_shutdown_system() {
        let mut app = App::new();

        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, shutdown_system);

        app.update();

        assert_eq!(
            0,
            app.world
                .query::<(&Player, &Coord2d, &AsciiSprite, &GameMap)>()
                .iter(&app.world)
                .len()
        );
    }
}
