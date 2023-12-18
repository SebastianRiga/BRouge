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

use bevy::prelude::Mut;
use bevy_ascii_terminal::Terminal;

use crate::core::position_2d::Position2d;

/// A container, usually consisting of a list of [View]s, which can be rendered on
/// demand. While the container groups the views and initiates their rendering through delegation,
/// the [View]s are responsible for their visual representation.
///
/// A [ViewGroup] makes up a specific section of arbitrary size on the screen, e.g.,
/// the in-game map, message box, dialog, etc.
///
/// # Examples
///
/// ```
/// struct Container {
///     pub elements: vec1[Element::default(); 80]
/// }
///
/// impl RenderContainer for Container {
///     fn render(&self, terminal: &mut Mut<Terminal>) {
///         for x in 0..80 {
///             self.elements[0].render_at([x, 1], terminal);
///         }
///     }
/// }
///
/// ...
///
/// struct Element {
///     pub symbol: Char
/// }
///
/// impl RenderElement for Element {
///     fn render_at(&self, position: impl Position2d, terminal: &mut Mut<Terminal>) {
///         terminal.put_char(position.to_array(), self.symbol);
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
/// * [Terminal]
/// * [View]
///
pub trait ViewGroup {
    /// Renders all elements which make up the container on screen using the passed
    /// [Terminal] reference.
    ///
    /// # Arguments
    ///
    /// * `terminal`: [Terminal] which handles the actual rendering.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// struct Container {
    ///     pub elements: vec1[Element::default(); 80]
    /// }
    ///
    /// impl RenderContainer for Container {
    ///     fn render(&self, terminal: &mut Mut<Terminal>) {
    ///         for x in 0..80 {
    ///             self.elements[0].render_at([x, 1], terminal);
    ///         }
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
    /// * [Terminal]
    ///
    fn render(&self, terminal: &mut Mut<Terminal>);
}

/// An element which can be rendered on demand given a specific position, usually as an isolated
/// part owned by a [ViewGroup], which supplies the position in its own respective
/// [ViewGroup::render] function.
///
/// It is responsible for it's own visual representation, e.g., markup, foreground and background,
/// only the actual render position within the game's world is passed in from the outside.
///
/// # Examples
///
/// ```
/// struct Container {
///     pub elements: vec1[Element::default(); 80]
/// }
///
/// impl RenderContainer for Container {
///     fn render(&self, terminal: &mut Mut<Terminal>) {
///         for x in 0..80 {
///             self.elements[0].render_at([x, 1], terminal);
///         }
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
/// * [Terminal]
/// * [ViewGroup]
/// * [Position2d]
///
pub trait View {
    /// Renders the element at the given `position` using the supplied `terminal` reference.
    ///
    /// # Arguments
    ///
    /// * `position`: The [Position2d] to render the element at.
    /// * `terminal`: [Terminal] which handles the actual rendering.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// #[derive(Component)]
    /// struct Element {
    ///     pub symbol: Char
    /// }
    ///
    /// impl RenderElement for Element {
    ///     fn render_at(&self, position: impl Position2d, terminal: &mut Mut<Terminal>) {
    ///         terminal.put_char(position.to_array(), self.symbol);
    ///     }
    /// }
    ///
    /// ...
    ///
    /// fn render_system(symbol_query: Query<Symbol>, &mut terminal: Terminal) {
    ///     for symbol in symbol_query.iter() {
    ///         symbol.render_at([x, y], terminal);
    ///     }
    /// }
    ///
    /// ```
    ///
    /// # About
    ///
    /// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
    ///
    /// Since: `0.15`
    ///
    /// # See also
    ///
    /// * [Terminal]
    /// * [Position2d]
    ///
    fn render_at(&self, position: impl Position2d, terminal: &mut Mut<Terminal>);
}
