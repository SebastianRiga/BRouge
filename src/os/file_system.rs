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

//! Provides functions to interface with the file system of the current platform's OS,
//! this includes all necessary fs operations like reading and writing files.
//!
//! # About
//!
//! Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
//!
//! Since: `0.1.5`
//!

#[cfg(not(target_family = "wasm"))]
use std::io::Read;

#[cfg(target_family = "wasm")]
use crate::js::local_storage;

/// Synchronously reads the the contents of the file at the passed `path`
/// and returns them as an `UTF-8` encoded [String].
///
/// # Arguments
///
/// * `path`: The path to the file.
///
/// returns: `UTF-8` encoded [String] representation of the file's content.
///
/// # Panics
///
/// * If the file can't be found.
/// * If the file can't be opened for any reason.
/// * If the file's content can't be converted to a valid `UTF-8` encoded [String], e.g.
/// reading binaries.
///
/// # Examples
///
/// ```
/// let json = file_system::load_file("config/window.json");
///
/// let window_config: WindowConfig = serde_json::from_str(&json).unwrap_or_else(|_| {
///    panic!("Unable to deserialize WindowConfig!");
/// })
///
/// ```
#[cfg(not(target_family = "wasm"))]
pub fn load_file(path: &str) -> String {
    let mut json = String::new();

    bevy::log::debug!("Loading file at: {}", path);

    std::fs::File::open(path)
        .unwrap_or_else(|error| {
            bevy::log::error!("{}", error.to_string());
            panic!("Unable to load file at: {}!", path);
        })
        .read_to_string(&mut json)
        .unwrap_or_else(|error| {
            bevy::log::error!("{}", error.to_string());
            panic!(
                "Unable to read file data, stream is not valid UTF-8 at {}!",
                path
            )
        });

    json
}

#[cfg(target_family = "wasm")]
/// Synchronously reads the the contents of the local storage with the passed `path` as the key
/// and returns them as an `UTF-8` encoded [String].
///
/// # Arguments
///
/// * `path`: The path to use as the key to read the local storage.
///
/// returns: `UTF-8` encoded [String] representation of the file's content.
///
/// # Examples
///
/// ```
/// let json = file_system::load_file("config/window.json");
///
/// let window_config: WindowConfig = serde_json::from_str(&json).unwrap_or_else(|_| {
///    panic!("Unable to deserialize WindowConfig!");
/// })
///
/// ```
pub fn load_file(path: &str) -> String {
    match local_storage::read_local_storage(path) {
        Some(json) => json,
        None => panic!("Unable to load file at: {}!", path),
    }
}
