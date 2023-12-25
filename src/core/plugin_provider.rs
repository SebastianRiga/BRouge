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

use bevy::app::Plugin;

/// Provides conversion functionality between configuration structures and and [Plugin]s,
/// e.g., a structure loaded form a local file and/or persistent settings before it is
/// converted to a [Plugin] in order to configure the game.
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
///
pub trait PluginProvider<P>
where
    P: Plugin,
{
    /// Creates, configures and returns the [Plugin] of type `P`, ready for registration with
    /// the bevy ecs.
    ///
    /// # Examples
    ///
    /// ```
    /// let window_config = WindowConfig::load();
    /// App::new()
    ///     ...
    ///     .add_plugins(DefaultPlugins.set(window_config.provide_plugin()))
    ///     ...
    ///     .run()
    /// ```
    ///
    fn provide_plugin(&self) -> P;
}
