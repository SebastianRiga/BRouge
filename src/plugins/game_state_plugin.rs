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
use crate::components::game_map::GameMap;
use crate::components::player::Player;
use crate::core::app_state::AppState;
use crate::core::dimension_2d::Dimension2d;
use crate::core::position_2d::Position2d;
use crate::core::view::{View, ViewGroup};
use crate::entities::player_bundle::PlayerBundle;
use crate::res::input_config::{InputConfig, InputType};
use crate::res::window_config::WindowConfig;

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
                (input_system, render_system).run_if(in_state(AppState::Game)),
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
    commands.spawn(GameMap::new(window_config.terminal_size()));
    PlayerBundle::spawn(&mut commands, window_config.terminal_size().center());
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
    window_config: Res<WindowConfig>,
    mut key_events: EventReader<KeyboardInput>,
    mut player_position_query: Query<&mut Coord2d, With<Player>>,
    mut exit_event: EventWriter<AppExit>,
) {
    for event in key_events.read() {
        if event.state == ButtonState::Released || event.key_code.is_none() {
            return;
        }

        if let Some(input) = input_config.parse_input(event.key_code.unwrap()) {
            info!("Received keyboard input event: {:?}", input);

            if input.is_movement_event() {
                handle_player_movement(
                    &input,
                    window_config.terminal_size(),
                    player_position_query.single_mut(),
                )
            }

            if input == InputType::Cancel {
                exit_event.send(AppExit)
            }
        }
    }
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
    mut terminal_query: Query<&mut Terminal>,
    game_map_query: Query<&GameMap>,
    render_query: Query<(&mut Coord2d, &mut AsciiSprite)>,
) {
    let mut terminal = terminal_query.single_mut();

    terminal.clear();

    game_map_query.single().render(&mut terminal);

    terminal.put_string([1, 1], "Hello bevy render world!");

    for (coord, sprite) in render_query.iter() {
        sprite.render_at(coord.as_array(), &mut terminal);
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
    input_type: &InputType,
    player_area: impl Dimension2d,
    mut player_position: Mut<Coord2d>,
) {
    match input_type {
        InputType::Up => {
            player_position.y = player_position.up(player_area.height() - 1).y;
        }
        InputType::Left => {
            player_position.x = player_position.left(0).x;
        }
        InputType::Right => {
            player_position.x = player_position.right(player_area.width() - 1).x;
        }
        InputType::Down => {
            player_position.y = player_position.down(0).y;
        }
        _ => {}
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

        {
            assert_eq!(1, app.world.query::<&Player>().iter(&app.world).len());
        }

        {
            let (position, sprite) = app
                .world
                .query::<(&Coord2d, &AsciiSprite)>()
                .single(&app.world);

            assert_eq!([50, 40], position.as_array());
            assert_eq!(&AsciiSprite::new('@', Color::ORANGE, Color::BLACK), sprite);
        }

        {
            let game_map = app.world.query::<&GameMap>().single(&app.world);
            assert_eq!(100, game_map.width);
            assert_eq!(80, game_map.height);
        }
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

        app.world.send_event(KeyboardInput {
            scan_code: 32,
            key_code: Some(KeyCode::W),
            state: ButtonState::Pressed,
            window,
        });

        app.update();

        assert_eq!(
            &Coord2d::new(50, 41),
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

        assert_eq!(
            &Coord2d::new(49, 41),
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

        assert_eq!(
            &Coord2d::new(49, 40),
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

        assert_eq!(
            &Coord2d::new(50, 40),
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
            &Coord2d::new(50, 40),
            app.world
                .query::<(&Coord2d, With<Player>)>()
                .single(&app.world)
                .0
        );
    }

    #[test]
    fn test_render_system() {
        let mut app = App::new();

        app.add_event::<KeyboardInput>();
        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, render_system);

        app.world
            .spawn(TerminalBundle::from(Terminal::new([100, 80])));

        app.update();

        let terminal = app.world.query::<&Terminal>().single(&app.world);

        assert_eq!('@', terminal.get_char([50, 40]));
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
