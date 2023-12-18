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

use bevy::prelude::{KeyCode, Resource};
use serde::Deserialize;

use crate::res::config_file::ConfigFile;

/// Serves as a translator between the raw periphery / hardware inputs from the user, e.g.,
/// keyboard inputs and mouse clicks, to events which can processed by the game in form of
/// [InputType]s.
///
/// This allows for the keybindings to be changed to the user's preference (on demand),
/// without the game having to perform any complex checks or updated a large amount
/// of information when bindings change.
///
/// The actual meaning of the InputType depends on the current context, e.g.,
/// an InputType::Up can mean an upwards movement of the player,
/// or changing the position of the selected option in a menu.
///
/// # Properties
///
/// * `up`: An upwards directed movement, e.g.,
/// moving the player up, moving the cursor up, moving a selection up.
/// * `left`: A leftwards directed movement, e.g.,
/// moving the player left, moving the cursor to the left, moving a selection to the left.
/// * `down`: A downwards directed movement, e.g.,
/// moving the player down, moving the cursor down, moving a selection down.
/// * `right`: A rightwards directed movement, e.g.,
/// moving the player right, moving the cursor to the right, moving a selection to the right.
/// * `cancel`: Cancelling a given action, e.g. closing a dialog, cancelling a choice, etc.
///
/// # Examples
///
/// ```
/// Json config file content:
///
/// {
///   "up": "W",
///   "left": "A",
///   "down": "S",
///   "right": "D",
///   "cancel": "Escape"
/// }
///
/// ...
///
/// let input_config = InputConfig::load();
///
/// assert_eq!(InputType::Up, input_config.parse_input(KeyCode::W));
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
/// * [KeyCode]
/// * [InputType]
///
#[derive(Debug, Copy, Clone, Deserialize, Resource)]
pub struct InputConfig {
    /// An upwards directed movement, e.g.,
    /// moving the player up, moving the cursor up, moving a selection up.
    pub up: KeyCode,
    /// A leftwards directed movement, e.g.,
    /// moving the player left, moving the cursor to the left, moving a selection to the left.
    pub left: KeyCode,
    /// A downwards directed movement, e.g.,
    /// moving the player down, moving the cursor down, moving a selection down.
    pub down: KeyCode,
    /// A rightwards directed movement, e.g.,
    /// moving the player right, moving the cursor to the right, moving a selection to the right.
    pub right: KeyCode,
    /// Cancelling a given action, e.g. closing a dialog, cancelling a choice, etc.
    pub cancel: KeyCode,
}

/// Serves as an abstraction layer between the raw user input in form of periphery events,
/// e.g., [KeyCode]s, and their respective meaning in the game, e.g. an upwards movement.
/// This allows for the keybindings to be changed to the user's preference, without the game
/// having to perform any complex checks or updated a large amount of information when bindings
/// change.
///
/// The conversion between periphery / hardware events and [InputType]s is the responsibility
/// of the [InputConfig], which loads the current mapping through the [ConfigFile] trait.
///
/// The actual meaning of the [InputType] depends on the current context, e.g., an [InputType::Up]
/// can mean an upwards movement of the player, or changing the position of the selected option
/// in a menu.
///
/// # Examples
///
/// ```
/// let input_config = InputConfig {
///     up: KeyCode::W,
///     left: KeyCode::A,
///     down: KeyCode::S,
///     right: KeyCode::D,
///     cancel: KeyCode::Escape,
/// }
///
/// assert_eq!(InputType::Right, input_config.parse_input(KeyCode::D));
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
/// * [InputConfig]
/// * [InputConfig::parse_input]
///
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum InputType {
    /// An upwards directed movement, e.g., moving the player up, moving the cursor up, moving a
    /// selection up.
    #[default]
    Up,
    /// A leftwards directed movement, e.g., moving the player left, moving the cursor to the left,
    /// moving a selection to the left.
    Left,
    /// A downwards directed movement, e.g., moving the player down, moving the cursor down,
    /// moving a selection down.
    Down,
    /// A rightwards directed movement, e.g., moving the player right, moving the cursor to
    /// the right, moving a selection to the right.
    Right,
    /// Cancelling a given action, e.g. closing a dialog, cancelling a choice, etc.
    Cancel,
}

impl InputConfig {
    /// Translates the passed [KeyCode] to its corresponding [InputEvent], which can then be
    /// handled by different systems of the game.
    ///
    /// # Arguments
    ///
    /// * `key_code`: The [KeyCode] which needs to be parsed
    ///
    /// returns: [Option]<[InputType]>
    ///
    /// # Examples
    ///
    /// ```
    /// let input_config = InputConfig {
    ///     up: KeyCode::W,
    ///     left: KeyCode::A,
    ///     down: KeyCode:S,
    ///     right: KeyCode::D,
    ///     cancel: KeyCode::Escape,
    /// };
    ///
    /// assert_eq!(InputType::UP, input_config.parse_input(KeyCode::W).unwrap()); // true
    /// assert_eq!(InputType::Left, input_config.parse_input(KeyCode::A).unwrap()); // true
    /// assert_eq!(InputType::Down, input_config.parse_input(KeyCode::S).unwrap()); // true
    /// assert_eq!(InputType::Right, input_config.parse_input(KeyCode::D).unwrap()); // true
    /// assert_eq!(InputType::Cancel, input_config.parse_input(KeyCode::Escape).unwrap()); // true
    /// assert_eq!(true, input_config.parse_input(KeyCode::F).is_none()); // false
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
    ///
    pub fn parse_input(&self, key_code: KeyCode) -> Option<InputType> {
        match key_code {
            _ if self.up == key_code => Some(InputType::Up),
            _ if self.left == key_code => Some(InputType::Left),
            _ if self.down == key_code => Some(InputType::Down),
            _ if self.right == key_code => Some(InputType::Right),
            _ if self.cancel == key_code => Some(InputType::Cancel),
            _ => None
        }
    }
}

impl ConfigFile for InputConfig {
    fn file_name() -> String {
        String::from("input.json")
    }
}

impl InputType {
    /// `True` if the respective [InputType] is a movement event of any kind,
    /// e.g. moving the player up or down, moving the cursor in a menu to the right, etc.
    ///
    /// # Arguments
    ///
    /// returns: `false` if the calling [InputType] is not a movement event.
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    pub fn is_movement_event(&self) -> bool {
        matches!(self, InputType::Up | InputType::Left | InputType::Down | InputType::Right)
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    const INPUT_CONFIG: InputConfig = InputConfig {
        up: KeyCode::W,
        left: KeyCode::A,
        down: KeyCode::S,
        right: KeyCode::D,
        cancel: KeyCode::Escape,
    };

    #[test]
    fn test_keycode_to_input_event_conversion() {
        assert_eq!(InputType::Up, INPUT_CONFIG.parse_input(KeyCode::W).unwrap());
        assert_eq!(InputType::Left, INPUT_CONFIG.parse_input(KeyCode::A).unwrap());
        assert_eq!(InputType::Right, INPUT_CONFIG.parse_input(KeyCode::D).unwrap());
        assert_eq!(InputType::Down, INPUT_CONFIG.parse_input(KeyCode::S).unwrap());
        assert_eq!(InputType::Cancel, INPUT_CONFIG.parse_input(KeyCode::Escape).unwrap());
    }

    #[test]
    fn test_movement_type_detection() {
        assert_eq!(true, InputType::Up.is_movement_event());
        assert_eq!(true, InputType::Left.is_movement_event());
        assert_eq!(true, InputType::Down.is_movement_event());
        assert_eq!(true, InputType::Right.is_movement_event());
        assert_eq!(false, InputType::Cancel.is_movement_event());
    }

    #[test]
    fn test_config_file_path() {
        assert_eq!(String::from("input.json"), InputConfig::file_name());
    }
}
