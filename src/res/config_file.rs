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

use std::env;
use std::path::PathBuf;

use bevy::log::debug;
use serde::de::DeserializeOwned;

use crate::os::file_system;

/// Allows structs which provide configuration data for the game to be loaded from local files.
/// This allows for a mutable default setup, which can be customized by the user.
///
/// # Examples
///
/// * [crate::res::window_config::WindowConfig]
/// * [crate::res::input_config::InputConfig]
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
/// # See also
///
/// * [file_system::load_file]
///
pub trait ConfigFile: DeserializeOwned {
    /// Specifies the actual name of the respective [ConfigFile] in the file system, so it can
    /// be loaded on demand.
    ///
    /// # Arguments
    ///
    /// returns: The name of the associated config file as a [String].
    ///
    /// # Examples
    ///
    /// ```
    /// let config_file = WindowConfig::new(...);
    ///
    /// assert_eq!("window.json", config_file.file_name());
    /// ```
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.1.5`
    ///
    fn file_name() -> String;

    /// Loads the configuration file with the set [ConfigFile::file_name] and attempts to
    /// deserialize the respective [ConfigFile] implementor from it.
    ///
    /// # Arguments
    ///
    /// returns: A new instance of the [ConfigFile] implementor.
    ///
    /// # Panics
    ///
    /// * If the loading of the file fails.
    /// * If the the [ConfigFile] implementor can't be serialized from
    /// the contents of the loaded file.
    ///
    /// # Examples
    ///
    /// ```
    /// let window_config: WindowConfig = WindowConfig::load();
    ///
    /// info!("{}", window_config);
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
    /// * [serde_json::from_str]
    /// * [file_system::load_file]
    ///
    fn load() -> Self {
        debug!(
            "Resolving file path for config file with name: {}",
            Self::file_name()
        );

        let path = resolve_config_file_path(Self::file_name());

        let json = file_system::load_file(&path);

        serde_json::from_str(&json).unwrap_or_else(|_| {
            panic!("Unable to load config file!");
        })
    }
}

/// Internal function to resolves the complete file path for the passed `file_name`
/// in the current system.
///
/// # Arguments
///
/// * `file_name`: The name of the file for which the path should be resolved.
///
/// returns: The complete path to the file as a [String].
///
/// # Note
///
/// When running this function in wasm, the [env::current_exe] isn't defined and the
/// resulting [PathBuf] falls back to the root of the project.
///
/// # Examples
///
/// ```
/// let file_path = resolve_config_file_path("window.json");
/// assert_eq!("../BRouge/config/window.json", file_path);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
fn resolve_config_file_path(file_name: String) -> String {
    let mut cwd = env::current_exe().unwrap_or_else(|_| PathBuf::new());

    cwd.pop();
    cwd.push("config");

    format!("{}/{}", cwd.display(), file_name)
}
