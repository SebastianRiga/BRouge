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

#![cfg(target_family = "wasm")]

//! Allows the reading and writing of [`key => value`] based data in the local storage of the
//! browser system.
//!
//! This takes the place of the [crate::os::file_system] to persist data, when the game
//! is running in the `wasm32-unknown-unknown` target.
//!
//! # Note
//!
//! Only available when the game is running in the `wasm32-unknown-unknown` target.
//!
//! # About
//!
//! Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
//!
//! Since: `0.1.5`
//!

use log::warn;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// wasm_bindgen definitions for the js_storage.
#[wasm_bindgen(module = "/web/bridge/storage.js")]
extern "C" {
    /// [wasm_bindgen] definition, which serves as the `Javascript` bridge
    /// for the [read_local_storage] function.
    ///
    /// Attempts to read the `value` from the local browser storage for the passed `key`,
    /// in form of a [JsValue], which needs to be parsed to the required `Rust` type.
    ///
    /// If an error occurs during the execution of the `Javascript` function, the error is
    /// also returned as a [JsValue] and needs to be parsed into a `Rust` type.
    ///
    /// # Arguments
    ///
    /// * `key`: The key for which the value should be retrieved.
    ///
    /// returns: [Result]<[JsValue], [JsValue]>
    ///
    /// # Examples
    ///
    /// ```
    /// match read_local_storage_actual(key) {
    ///     Ok(js_value) => {
    ///         match js_value.as_string() {
    ///             None => {
    ///                 web_sys::console::error_1(&js_value);
    ///                 warn!("Unable fetch data from local storage for key: {}!", key);
    ///                 None
    ///             }
    ///             Some(json) => Some(json)
    ///         }
    ///     }
    ///     Err(js_error) => {
    ///         web_sys::console::error_1(&js_error);
    ///         warn!("Unable fetch data from local storage for key: {}!", key);
    ///         None
    ///     }
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
    /// * [JsValue]
    ///
    #[wasm_bindgen(catch)]
    fn read_local_storage_actual(key: &str) -> Result<JsValue, JsValue>;

    /// [wasm_bindgen] definition, which serves as the `Javascript` bridge
    /// for the [write_local_storage] function.
    ///
    /// Attempts to store the passed `value` int the local browser storage with the given `key`.
    /// If the execution of the `Javascript` function is successful, the [Result] contains no
    /// value.
    ///
    /// If an error occurs during the execution of the `Javascript` function, the error is
    /// also returned as a [JsValue] and needs to be parsed into a `Rust` type.
    ///
    /// # Arguments
    ///
    /// * `key`: The key with which the value should be stored.
    /// * `value`: The string value to store.
    ///
    /// returns: [Result]<(), [JsValue]>
    ///
    /// # Examples
    ///
    /// ```
    /// match write_local_storage_actual(key, value) {
    ///     Ok(_) => true,
    ///     Err(js_error) => {
    ///         web_sys::console::error_1(&js_error);
    ///         false
    ///     }
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
    /// * [JsValue]
    ///
    #[wasm_bindgen(catch)]
    fn write_local_storage_actual(key: &str, value: &str) -> Result<(), JsValue>;
}

/// Attempts to read the string value from the local browser storage for the passed `key`.
/// If no `value` for the supplied `key` exist, [None] is returned.
///
/// # Arguments
///
/// * `key`: The key for which the value should be retrieved.
///
/// returns: [Option]<[String]>
///
/// # Examples
///
/// ```
/// match js_storage::read_local_storage("config/window.json") {
///     Ok(json) => Use the storage contents
///     None => // Fallback or panic
/// }
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
pub fn read_local_storage(key: &str) -> Option<String> {
    match read_local_storage_actual(key) {
        Ok(js_value) => match js_value.as_string() {
            None => {
                web_sys::console::error_1(&js_value);
                warn!("Unable fetch data from local storage for key: {}!", key);
                None
            }
            Some(json) => Some(json),
        },
        Err(js_error) => {
            web_sys::console::error_1(&js_error);
            warn!("Unable fetch data from local storage for key: {}!", key);
            None
        }
    }
}

/// Stores the passed string `value` with the given `key` in the local browser storage.
///
/// # Arguments
///
/// * `key`: The key with which the value should be stored.
/// * `value`: The string value to store.
///
/// returns: bool `true` if the value was stored successfully and `false` otherwise.
///
/// # Examples
///
/// ```
/// let write_result = local_storage::write_local_storage("foo", "bar");
///
/// if write_result {
///     let read_value = local_storage::read_local_storage("foo");
///
///     assert_eq!(Some("bar"), read_value);
/// }
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.5`
///
pub fn write_local_storage(key: &str, value: &str) -> bool {
    match write_local_storage_actual(key, value) {
        Ok(_) => true,
        Err(js_error) => {
            web_sys::console::error_1(&js_error);
            false
        }
    }
}
