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

use bevy::prelude::Resource;
use bevy::utils::default;
use bevy::window::{MonitorSelection, Window, WindowPlugin, WindowPosition, WindowResolution};
use serde::Deserialize;

use crate::core::constants;
use crate::core::dimension_2d::Dimension2d;
use crate::core::plugin_provider::PluginProvider;
use crate::res::config_file::ConfigFile;

/// A [bevy::prelude::Resource] for configuring and creating the display [Window] of the game.
///
/// It uses the [PluginProvider] trait for conversion into the
/// bevy [WindowPlugin], which can then be insert directly into the ECS.
///
/// It is usually not instantiated directly, but deserialized from a configuration file shipped
/// with the game. This allows the user to customize the graphics of the game and persist the
/// changes. See the [ConfigFile] trait for more information.
///
/// # Properties
/// * `width`: The width of the [Window].
/// * `height`: The height of the [Window].
/// * `resizeable`: If the [Window] is resizable.
/// * `position`: _(Private)_ The monitor position of the resulting [Window] as an `i32` due to serialization
/// constraints. See the [WindowConfig::get_position] function for the mapping table.
///
/// # Examples
///
/// ## Creation
///
/// ```
/// let window_config = WindowConfig::load();
///
/// App::new()
///     ...
///     .add_plugins(DefaultPlugins.set(window_config.provide_plugin()))
///     .insert_resource(window_config)
///     ...
///     .run()
/// ```
///
/// ## Usage
///
/// ```
/// fn some_system(window_config: Res<InputConfig>, ...) {
///     /// Use the WindowConfig...
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
/// * [Window]
/// * [WindowPosition]
/// * [MonitorSelection]
/// * [WindowPlugin]
/// * [PluginProvider]
///
#[derive(Debug, Clone, Deserialize, Resource)]
pub struct WindowConfig {
    /// The width of the [Window].
    pub width: i32,
    /// The height of the [Window].
    pub height: i32,
    /// If the [Window] is resizable.
    pub resizeable: bool,
    /// The monitor position of the resulting [Window] as an `i32` due to serialization
    /// constraints. See the [WindowConfig::get_position] function for the mapping table.
    position: i32,
}

impl WindowConfig {
    /// Maps the `i32` definition of the [WindowConfig::position] property to its respective
    /// bevy [WindowPosition].
    ///
    /// The result can then be used to position a bevy [Window] during its initialization.
    ///
    /// # Note
    ///
    /// If the `i32` position can't be mapped, [WindowPosition::Automatic] is returned.
    ///
    /// # Arguments
    ///
    /// returns: [WindowPosition]
    ///
    /// # Examples
    ///
    /// Creating a new [WindowConfig] and mapping its position:
    ///
    /// ```
    /// let window_config = WindowConfig::new([800, 640], true, 2);
    /// window_config.getPosition(); // WindowConfig::Centered(MonitorSelection::Primary)
    /// ```
    ///
    /// Mapping an unknown position:
    ///
    /// ```
    /// let window_config = WindowConfig::new([800, 640], true, -1);
    /// window_config.getPosition(); // WindowConfig::Automatic
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    /// # See also
    /// * [WindowPosition]
    /// * [MonitorSelection]
    ///
    pub fn get_position(&self) -> WindowPosition {
        match self.position {
            0 => WindowPosition::Automatic,
            1 => WindowPosition::Centered(MonitorSelection::Current),
            2 => WindowPosition::Centered(MonitorSelection::Primary),
            _ => WindowPosition::Automatic,
        }
    }

    /// Calculates the display dimension of the [bevy_ascii_terminal::Terminal]
    /// in the resulting [Window].
    ///
    /// # See also
    /// * [constants::TILES_PER_PIXEL]
    /// * [Dimension2d]
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn terminal_size(&self) -> impl Dimension2d {
        [
            self.width / constants::TILES_PER_PIXEL,
            self.height / constants::TILES_PER_PIXEL,
        ]
    }
}

impl PluginProvider<WindowPlugin> for WindowConfig {
    fn provide_plugin(&self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: String::from(constants::TITLE),
                resolution: WindowResolution::new(self.width as f32, self.height as f32),
                resizable: self.resizeable,
                position: self.get_position(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }
    }
}

impl ConfigFile for WindowConfig {
    fn file_name() -> String {
        String::from("window.json")
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    impl WindowConfig {
        pub fn new(dimension: impl Dimension2d, resizeable: bool, position: i32) -> Self {
            Self {
                width: dimension.width(),
                height: dimension.height(),
                resizeable,
                position,
            }
        }
    }

    #[test]
    fn test_window_position_mapping() {
        assert_eq!(
            WindowPosition::Automatic,
            WindowConfig::new([800, 640], false, 0).get_position()
        );
        assert_eq!(
            WindowPosition::Centered(MonitorSelection::Current),
            WindowConfig::new([800, 640], false, 1).get_position()
        );
        assert_eq!(
            WindowPosition::Centered(MonitorSelection::Primary),
            WindowConfig::new([800, 640], false, 2).get_position()
        );
        assert_eq!(
            WindowPosition::Automatic,
            WindowConfig::new([800, 640], false, -1).get_position()
        );
    }

    #[test]
    fn test_terminal_size_calculation() {
        assert_eq!(
            [100, 80],
            WindowConfig::new([800, 640], false, 0)
                .terminal_size()
                .as_array()
        )
    }

    #[test]
    fn test_plugin_provision() {
        let window_config = WindowConfig::new([800, 640], false, 0);

        let primary_window: Window = window_config.provide_plugin().primary_window.unwrap();

        assert_eq!(constants::TITLE, primary_window.title);
        assert_eq!(
            primary_window.resolution,
            WindowResolution::new(window_config.width as f32, window_config.height as f32,)
        );
        assert_eq!(window_config.resizeable, primary_window.resizable);
        assert_eq!(window_config.get_position(), primary_window.position);
        assert_eq!(true, primary_window.fit_canvas_to_parent);
    }

    #[test]
    fn test_config_file_name() {
        assert_eq!("window.json", WindowConfig::file_name());
    }
}
