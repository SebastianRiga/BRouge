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

use bevy::prelude::{Commands, DetectChangesMut, Entity, Query, Res, ResMut, With};
use log::debug;

use crate::components::state_label::GameStateLabel;
use crate::core::dimension_2d::Dimension2d;
use crate::entities::monster_factory::MonsterFactory;
use crate::entities::player_factory::PlayerFactory;
use crate::plugins::states::GameTurnState;
use crate::res::window_config::WindowConfig;
use crate::ui::game_map::GameMap;
use crate::ui::tile_map_layout_generator::BaseTileMapGenerator;

/// System which is run when the game's state machine changes into the
/// [AppState::Game] state to spawn all required  `entities`.
///
/// # Arguments
///
/// * `commands`: [Commands] queue required to spawn the necessary `entities`.
/// * `window_config`: [WindowConfig] resource required to check the bounds of the game's
/// window during the `entity` creation.
///
/// returns: ()
///
/// # Panics
///
/// * If the [WindowConfig] resource can't be retrieved from the ECS.
/// * If no starting position for the `player entity` can be determined.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
pub fn startup_system(mut commands: Commands, window_config: Res<WindowConfig>) {
    commands.insert_resource(GameTurnState::Player);

    let game_map = GameMap::new(&window_config.terminal_size(), &BaseTileMapGenerator);
    let starting_position = game_map
        .rooms()
        .first()
        .expect("ECS -> Systems -> startup_system -> Unable to find a starting position for the player entity!")
        .center();

    PlayerFactory::spawn(&mut commands, &starting_position);

    for room in game_map.rooms().iter().skip(1) {
        MonsterFactory::spawn_mended(&mut commands, &room.center());
    }

    commands.spawn(game_map).insert(GameStateLabel);
}

/// Resets the [GameTurnState] back to [GameTurnState::Player] after the `NPC entity systems` have run, giving
/// control back to the player.
///
/// # Arguments
///
/// * `in_game_state`: The [InGameTurnState] [bevy::ecs::prelude::Resource] to update.
///
/// returns: ()
///
/// # Panics
///
/// If the [GameTurnState] resource can't be retrieved from the ECS.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.9`
///
pub fn npc_turn_end_system(mut in_game_state: ResMut<GameTurnState>) {
    // Only reset the resource if necessary for performance.
    if in_game_state.set_if_neq(GameTurnState::Player) {
        debug!(
            "ECS -> Systems -> npc_turn_end_system -> Setting GameTurnState back to {}",
            GameTurnState::Player
        );
    }
}

/// Clean up system, which is run when the game's state machine is leaving the
/// [AppState::Game] state.
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
/// # Panics
///
/// * If the [Query] to retrieve the state's `entities` fails.
/// * If the de-spawning of the respective `entities` fails.
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
///
pub fn shutdown_system(
    mut commands: Commands,
    entities_query: Query<Entity, With<GameStateLabel>>,
) {
    for entity in entities_query.iter() {
        commands.get_entity(entity).unwrap().despawn();
    }

    commands.remove_resource::<GameTurnState>();
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use crate::components::ascii_sprite::AsciiSprite;
    use crate::components::coord_2d::Coord2d;
    use crate::components::player::Player;
    use crate::core::position_2d::Position2d;

    use super::*;

    #[test]
    fn test_startup_system() {
        let mut app = App::new();

        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.add_systems(Startup, startup_system);

        app.update();

        assert_eq!(1, app.world.query::<&Player>().iter(&app.world).len());

        assert_eq!(
            &AsciiSprite::new('@', Color::ORANGE, Color::BLACK),
            app.world
                .query_filtered::<&AsciiSprite, With<Player>>()
                .single(&app.world)
        );

        assert_eq!(
            app.world
                .query_filtered::<&Coord2d, With<Player>>()
                .single(&app.world)
                .as_array(),
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
    fn test_npc_turn_end_system() {
        let mut app = App::new();

        app.insert_resource(WindowConfig::new([800, 640], true, 1));
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, npc_turn_end_system);

        app.update();

        assert_eq!(
            Some(&GameTurnState::Player),
            app.world.get_resource::<GameTurnState>()
        );

        app.world
            .resource_mut::<GameTurnState>()
            .set_if_neq(GameTurnState::Npc);

        assert_eq!(
            Some(&GameTurnState::Npc),
            app.world.get_resource::<GameTurnState>()
        );

        app.update();

        assert_eq!(
            Some(&GameTurnState::Player),
            app.world.get_resource::<GameTurnState>()
        );
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
                .query_filtered::<Entity, With<GameStateLabel>>()
                .iter(&app.world)
                .len()
        );

        assert_eq!(None, app.world.get_resource::<GameTurnState>())
    }
}
