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
use log::info;

use crate::components::fov::Fov;
use crate::core::position_2d::Position2d;
use crate::ui::tile::Tile;
use crate::ui::tile_map::TileMap;

///
///
/// # Arguments
///
/// * `fov`:
/// * `position`:
/// * `area`:
///
/// returns: ()
///
/// # Examples
///
/// ```
///
/// ```
pub fn field_of_view<T: Tile>(
    fov: &mut Mut<Fov>,
    position: &impl Position2d,
    area: &mut Mut<impl TileMap<T>>,
) {
    if !fov.is_dirty {
        return;
    }

    info!(
        "Calculating field of view with a radius of '{:?}' at position ({:?}, {:?}).",
        fov.radius,
        position.x_coordinate(),
        position.y_coordinate()
    );

    fov.clear();
    area.reset_visible_tiles();

    area.mark_tile_as_seen(position);
    fov.push_position(position);

    for x in (position.x_coordinate() - fov.radius)..(position.x_coordinate() + fov.radius) {
        for y in (position.y_coordinate() - fov.radius)..(position.y_coordinate() + fov.radius) {
            let target = [x, y];

            if calculate_distance(position, &target) < fov.radius
                && area.is_in_bounds(&target)
                && is_in_line_of_sight(position, &target, area)
            {
                area.mark_tile_as_seen(&target);
                area.mark_tile_as_visible(&target);
                fov.push_position(&target);
            }
        }
    }

    fov.is_dirty = false
}

///
///
/// # Arguments
///
/// * `start`:
/// * `end`:
///
/// returns: i32
///
/// # Examples
///
/// ```
///
/// ```
fn calculate_distance(start: &impl Position2d, end: &impl Position2d) -> i32 {
    let [x_delta, y_delta] = end.delta(start);
    (((x_delta * x_delta) + (y_delta * y_delta)) as f64)
        .sqrt()
        .floor() as i32
}

///
///
/// # Arguments
///
/// * `start`:
/// * `end`:
/// * `area`:
///
/// returns: bool
///
/// # Examples
///
/// ```
///
/// ```
fn is_in_line_of_sight<T: Tile>(
    start: &impl Position2d,
    end: &impl Position2d,
    area: &Mut<impl TileMap<T>>,
) -> bool {
    let mut delta = start.delta(end);
    let delta_signed = get_offset_delta(&delta);
    delta = [delta.x_coordinate().abs(), delta.y_coordinate().abs()];

    if delta.x_coordinate() > delta.y_coordinate() {
        calculate_horizontal_slope_in_line_of_sight(start, end, &delta, &delta_signed, area)
    } else {
        calculate_vertical_slope_in_line_of_sight(start, end, &delta, &delta_signed, area)
    }
}

///
///
/// # Arguments
///
/// * `start`:
/// * `end`:
/// * `delta`:
/// * `delta_signed`:
/// * `area`:
///
/// returns: bool
///
/// # Examples
///
/// ```
///
/// ```
fn calculate_horizontal_slope_in_line_of_sight<T: Tile>(
    start: &impl Position2d,
    end: &impl Position2d,
    delta: &impl Position2d,
    delta_signed: &impl Position2d,
    area: &Mut<impl TileMap<T>>,
) -> bool {
    let mut x = end.x_coordinate();
    let mut y = end.y_coordinate();
    let mut theta = (delta.y_coordinate() * 2) - delta.x_coordinate();

    loop {
        if theta >= 0 {
            y += delta_signed.y_coordinate();
            theta -= delta.x_coordinate() * 2;
        }

        x += delta_signed.x_coordinate();
        theta += delta.y_coordinate() * 2;

        if start.as_array() == [x, y] {
            return true;
        }

        if area.tile_has_collision(&[x, y]) {
            break;
        }
    }

    false
}

///
///
/// # Arguments
///
/// * `start`:
/// * `end`:
/// * `delta`:
/// * `delta_signed`:
/// * `area`:
///
/// returns: bool
///
/// # Examples
///
/// ```
///
/// ```
fn calculate_vertical_slope_in_line_of_sight<T: Tile>(
    start: &impl Position2d,
    end: &impl Position2d,
    delta: &impl Position2d,
    delta_signed: &impl Position2d,
    area: &Mut<impl TileMap<T>>,
) -> bool {
    let mut x = end.x_coordinate();
    let mut y = end.y_coordinate();
    let mut theta = (delta.x_coordinate() * 2) - delta.y_coordinate();

    loop {
        if theta >= 0 {
            x += delta_signed.x_coordinate();
            theta -= delta.y_coordinate() * 2;
        }

        y += delta_signed.y_coordinate();
        theta += delta.x_coordinate() * 2;

        if start.as_array() == [x, y] {
            return true;
        }

        if area.tile_has_collision(&[x, y]) {
            break;
        }
    }

    false
}

///
///
/// # Arguments
///
/// * `position`:
///
/// returns: [i32; 2]
///
/// # Examples
///
/// ```
///
/// ```
fn get_offset_delta(position: &impl Position2d) -> [i32; 2] {
    [
        if position.x_coordinate() >= 0 { 1 } else { -1 },
        if position.y_coordinate() >= 0 { 1 } else { -1 },
    ]
}
