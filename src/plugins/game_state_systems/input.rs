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

use bevy::app::AppExit;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::log::debug;
use bevy::prelude::{
    DetectChangesMut, EventReader, EventWriter, Mut, Query, Res, ResMut, With, Without,
};

use crate::components::collision::Collision;
use crate::components::coord_2d::Coord2d;
use crate::components::fov::Fov;
use crate::ui::game_map::GameMap;
use crate::components::player::Player;
use crate::plugins::states::GameTurnState;
use crate::res::input_config::{InputConfig, InputType};
use crate::ui::tile::Tile;
use crate::ui::tile_map::TileMap;

/// System to handle user's input through the keyboard.
///
/// # Arguments
///
/// * `input_config`: [InputConfig] required to recognize the user's input.
/// * `game_map_query`: [Query] required to retrieve the [GameMap], which is used to check for collision.
/// * `exit_event`: [EventWriter] to send the [AppExit] event to the game's engine in order to close the game.
/// * `in_game_state`: [GameTurnState] to update, when the player makes a valid movement, in order to pass the
/// turn to the `NPC entities`.
/// * `key_events`: [EventReader] stream of [KeyboardInput] events required to parse the user's input.
/// * `player_query`: [Query] to retrieve the position of the `player entity`, required to move him according
/// to the user's input (if applicable).
/// * `collision_entity_query`: [Query] to retrieve the positions of the `entities` which have collision.
///
/// returns: ()
///
/// # Panics
///
/// * If any of the resources required by the system aren't available through the ECS.
/// * If any of the [Query] calls fail, i.e., the components required by the system can't be retrieved from the ECS.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
pub fn keyboard_input_system(
    input_config: Res<InputConfig>,
    game_map_query: Query<&GameMap>,
    mut exit_event: EventWriter<AppExit>,
    mut turn_state: ResMut<GameTurnState>,
    mut key_events: EventReader<KeyboardInput>,
    mut player_query: Query<(&mut Fov, &mut Coord2d), With<Player>>,
    collision_entity_query: Query<&Coord2d, (With<Collision>, Without<Player>)>,
) {
    for event in key_events.read() {
        if event.state == ButtonState::Released || event.key_code.is_none() {
            return;
        }

        if let Some(key_code) = event.key_code {
            if let Some(input) = input_config.parse_input(key_code) {
                debug!("ECS -> Systems -> keyboard_input_system -> Received keyboard input event: {:?}", input);

                let (mut fov, mut position) = player_query.get_single_mut().expect(
                    "ECS -> Systems -> keyboard_input_system -> \
                    Unable to retrieve player {Fov} and {Coord2d} components!",
                );

                let map = game_map_query.get_single().expect(
                    "ECS -> Systems -> keyboard_input_system -> Unable to retrieve {GameMap} component!"
                );

                if input.is_movement_event() {
                    turn_state.set_if_neq(handle_player_movement(
                        &input,
                        &mut fov,
                        map,
                        &mut position,
                        &collision_entity_query.iter().collect(),
                    ));
                }

                if input == InputType::Cancel {
                    exit_event.send(AppExit)
                }
            }
        }
    }
}

/// Internal function to update the `player entities` positional component according to the passed `input_type`
/// within the set `player_area` and the given `entity_collision_positions`.
///
/// If the `player entity` is moved, the passed associated `fov` is also marked as dirty to trigger a recalculation.
///
/// # Arguments
///
/// * `input_type`: The movement [InputType] according to which the `player_position` will be manipulated.
/// * `player_fov`: The `field of view` of the `player entity`.
/// * `tile_map`: The [TileMap] on which the `player` moves, required for bounds and collision checking.
/// * `player_position`: The [Coord2d] ecs [bevy::prelude::Component] of the `player` `entity`.
/// * `entity_collision_positions`: List of all positions on the current map, which are occupied by an `entity`
/// with collision.
///
/// returns: [GameTurnState]
///
/// # Examples
///
/// ```
/// let mut player_fov = Fov::new(8);
/// let tile_map = TileMapImpl::new(...);
/// let mut player_position = Coord2d::new(40, 25);
/// handle_player_movement(InputType::Up, &player_fov, &map, &player_position, &Vec::new());
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
fn handle_player_movement<T: Tile>(
    input_type: &InputType,
    player_fov: &mut Mut<Fov>,
    tile_map: &impl TileMap<T>,
    player_position: &mut Mut<Coord2d>,
    entity_collision_positions: &Vec<&Coord2d>,
) -> GameTurnState {
    let new_position = match input_type {
        InputType::Up => player_position.up(tile_map.height() - 1),
        InputType::Left => player_position.left(0),
        InputType::Right => player_position.right(tile_map.width() - 1),
        InputType::Down => player_position.down(0),
        _ => Coord2d::from_position(&[player_position.x, player_position.y]),
    };

    let player_collides_with_entity = entity_collision_positions
        .iter()
        .find(|coord2d: &&&Coord2d| ***coord2d == new_position)
        .is_some();

    if tile_map.tile_has_collision(&new_position) || player_collides_with_entity {
        return GameTurnState::Npc;
    }

    if new_position != **player_position {
        player_fov.is_dirty = true;
        player_position.x = new_position.x;
        player_position.y = new_position.y;
    }

    GameTurnState::Npc
}

#[cfg(test)]
mod tests {
    use bevy::app::{App, Startup, Update};
    use bevy::prelude::{Component, KeyCode};

    use crate::plugins::game_state_systems::lifecycle::startup_system;
    use crate::res::window_config::WindowConfig;

    use super::*;

    #[derive(Component)]
    struct DummyComponent;

    #[test]
    fn test_keyboard_input_system() {
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
        app.add_systems(Update, keyboard_input_system);

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
}
