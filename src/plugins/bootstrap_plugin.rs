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

use bevy::app::{App, Plugin, PluginGroup, PreStartup};
use bevy::DefaultPlugins;
use bevy::prelude::{Commands, Res};
use bevy_ascii_terminal::{AutoCamera, Terminal, TerminalBundle, TerminalPlugin, ToWorld};

use crate::core::app_state::AppState;
use crate::core::dimension_2d::Dimension2d;
use crate::core::plugin_provider::PluginProvider;
use crate::plugins::game_state_plugin::GameStatePlugin;
use crate::res::config_file::ConfigFile;
use crate::res::input_config::InputConfig;
use crate::res::window_config;
use crate::res::window_config::WindowConfig;

/// Initial entrypoint [Plugin] of the game.
///
/// It creates all other [Plugin]s, global [bevy::prelude::Resource]s, the [AppState] and initial scaffolding
/// systems and ensures that they are operational and added to the created [App].
///
/// After the bootstrapping process, it passes the control to the state [Plugin]
/// corresponding to the default [AppState].
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
/// # See also
///
/// * [DefaultPlugins]
/// * [TerminalPlugin]
/// * [WindowConfig]
/// * [InputConfig]
/// * [AppState]
///
pub struct BootstrapPlugin;

impl Plugin for BootstrapPlugin {
    fn build(&self, app: &mut App) {
        let window_config = window_config::WindowConfig::load();

        // The order of the added game components is important:
        // 1. Standard and base plugins
        // 2. Resources
        // 3. Bootstrap systems
        // 4. States
        // 5. All other state plugins
        app
            .add_plugins(DefaultPlugins.set(window_config.provide_plugin()))
            .add_plugins(TerminalPlugin)
            .insert_resource(window_config)
            .insert_resource(InputConfig::load())
            .add_systems(PreStartup, startup_system)
            .add_state::<AppState>()
            .add_plugins(GameStatePlugin);
    }

    fn name(&self) -> &str {
        "BRouge: Bootstrapper"
    }

    fn is_unique(&self) -> bool {
        true
    }
}

/// # @System
///
/// Sets-up all of the game's required base components and bundles, 
/// e.g., the world, rendering and camera.
///
/// schedule: [PreStartup]
///
/// # Arguments
///
/// * `commands`: A [bevy::ecs::system::Command] queue to perform impactful changes to the [bevy::prelude::World].
/// * `window_config`: [ConfigFile] implementor required to setup the game's window.
///
/// returns: ()
///
/// # Examples
///
/// ```
/// fn build(&self, app: &mut App) {
///     app.add_systems(PreStartup, startup_system)
/// }
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
/// * [bevy::ecs::system::Command]
/// * [bevy::prelude::World]
/// * [WindowConfig]
///
fn startup_system(mut commands: Commands, window_config: Res<WindowConfig>) {
    let terminal_size: [i32; 2] = window_config
        .terminal_size()
        .as_array();

    commands.spawn((
        TerminalBundle::from(Terminal::new(terminal_size)),
        ToWorld::default(),
        AutoCamera
    ));
}
