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

use bevy::log::debug;

use crate::components::fov::Fov;
use crate::core::position_2d::Position2d;
use crate::ui::tile::Tile;
use crate::ui::tile_map::TileMap;

/// Calculates the `field of view` for the passed `position` on the given `map` and updates the supplied `fov`
/// [bevy::prelude::Component] with the result.
///
/// # Arguments
///
/// * `fov`: The [Fov] component to update.
/// * `position`: The center [Position2d] starting from which the `field of view` will be calculated.
/// * `map`: The [TileMap] on which the `field of view` is calculated. Required for bounds and collision checking.
///
/// returns: ()
///
/// # Examples
///
/// Given the following [TileMap]:
///
/// ```text
/// ##############################
/// |                           |
/// |          1         #      |
/// |                    #2     |
/// |                           |
/// ##############################
/// ```
///
/// The `fov` can now be calculated as follows:
///
/// ```
/// let fov = Fov::new(8);
/// let position = (5, 5);
/// let map = TileMapImpl::new(...);
///
/// field_of_view(&fov, &position, &map);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
pub fn field_of_view<T: Tile>(
    fov: &mut Fov,
    position: &impl Position2d,
    map: &mut impl TileMap<T>,
) {
    if !fov.is_dirty {
        return;
    }

    debug!(
        "Calculating field of view with {:?} at {:?}.",
        fov.radius, position
    );

    fov.clear();
    fov.push_position(position);

    for x in (position.x_coordinate() - fov.radius)..(position.x_coordinate() + fov.radius) {
        for y in (position.y_coordinate() - fov.radius)..(position.y_coordinate() + fov.radius) {
            let target = [x, y];

            if calculate_distance(position, &target) < fov.radius
                && map.is_in_bounds(&target)
                && is_in_line_of_sight(position, &target, map)
            {
                fov.push_position(&target);
            }
        }
    }

    fov.is_dirty = false
}

/// Calculates the step distance between the passed `start` and `end` [Position2d].
///
/// # Arguments
///
/// * `start`: The [Position2d] from which the distance should be calculated.
/// * `end`: The [Position2d] to which the distance should be calculated.
///
/// returns: i32 - The distance between `start` and `end` [Position2d] in steps.
///
/// # Examples
///
/// ```
/// let start_position = (2, 2);
/// let end_position = (10, 10);
///
/// assert_eq!(5, calculate_distance(&start_position, &end_position);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
fn calculate_distance(start: &impl Position2d, end: &impl Position2d) -> i32 {
    let [x_delta, y_delta] = end.delta(start);
    (((x_delta * x_delta) + (y_delta * y_delta)) as f64)
        .sqrt()
        .floor() as i32
}

/// Checks if the passed `end` position is in the line of sight of the set `start` position on the given [TileMap].
///
/// # Arguments
///
/// * `start`: The starting [Position2d], from which the line of sight should be checked.
/// * `end`: The ending [Position2d], to which the line of sight should be checked.
/// * `map`: The [TileMap] on which the slope is calculated. Required for bounds and collision checking.
///
/// returns: bool - `true` if the `end` position is in the line of sight of the `start` position and `false` otherwise.
///
/// # Examples
///
/// Given [TileMap]:
///
/// ```text
/// ##############################
/// |                           |
/// |          1         #      |
/// |                    #2     |
/// |                           |
/// ##############################
/// ```
///
/// The check will look something like this:
///
/// ```
/// let map = TileMapImpl::new(...);
///
/// let start_position = (5, 5);
/// let end_position = (10, 4);
///
/// assert!(!is_in_line_of_sight(&start_position, &end_position, &map));
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
fn is_in_line_of_sight<T: Tile>(
    start: &impl Position2d,
    end: &impl Position2d,
    map: &impl TileMap<T>,
) -> bool {
    let mut delta = start.delta(end);
    let delta_signed = get_sign_multiplier(&delta);
    delta = [delta.x_coordinate().abs(), delta.y_coordinate().abs()];

    if delta.x_coordinate() > delta.y_coordinate() {
        calculate_horizontal_slope_in_line_of_sight(start, end, &delta, &delta_signed, map)
    } else {
        calculate_vertical_slope_in_line_of_sight(start, end, &delta, &delta_signed, map)
    }
}

/// Calculates the horizontal slope based line of sight between the `start` and `end` [Position2d], in order to
/// check if the  `end` position is in line of sight of the `start` position.
///
/// # Arguments
///
/// * `start`: The starting [Position2d] to which the slope is calculated.
/// * `end`: The ending [Position2d] from which the slope is calculated.
/// * `delta`: The delta between the `start` and `end` [Position2d].
/// * `delta_signed`: The sign-multiplier for the slopes `x` and `y-coordinates`.
/// * `map`: The [TileMap] on which the slope is calculated. Required for bounds and collision checking.
///
/// returns: bool - `true` if a horizontal slope can be calculated from the `end` to the `start` [Position2d],
/// without going out of bounds or hitting a position with collision.
///
/// # Examples
///
/// Given [TileMap]:
///
/// ```text
/// ##############################
/// |                           |
/// |          1                |
/// |                     2     |
/// |                           |
/// ##############################
/// ```
///
/// The calculation will look something like this:
///
/// ```
/// let map = TileMapImpl::new(...);
///
/// let start_position = (5, 5);
/// let end_position = (10, 4);
///
/// let delta = start_position.delta(&end_position);
///
/// assert!(calculate_horizontal_slope_in_line_of_sight(
///     &start_position,
///     &end_position,
///     &delta,
///     &get_sign_multiplier(&delta),
///     &map
/// );
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
fn calculate_horizontal_slope_in_line_of_sight<T: Tile>(
    start: &impl Position2d,
    end: &impl Position2d,
    delta: &impl Position2d,
    delta_signed: &impl Position2d,
    map: &impl TileMap<T>,
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

        if map.tile_has_collision(&[x, y]) {
            break;
        }
    }

    false
}

/// Calculates the vertical slope based line of sight between the `start` and `end` [Position2d],
/// in order to check if the  `end` position is in line of sight of the `start` position.
///
/// # Arguments
///
/// * `start`: The starting [Position2d] to which the slope is calculated.
/// * `end`: The ending [Position2d] from which the slope is calculated.
/// * `delta`: The delta between the `start` and `end` [Position2d].
/// * `delta_signed`: The sign-multiplier for the slopes `x` and `y-coordinates`.
/// * `map`: The [TileMap] on which the slope is calculated. Required for bounds and collision checking.
///
/// returns: bool - `true` if a vertical slope can be calculated from the `end` to the `start` [Position2d],
/// without going out of bounds or hitting a position with collision.
///
/// # Examples
///
/// Given [TileMap]:
///
/// ```text
/// ##########
/// |       |
/// |       |
/// |       |
/// |   1   |
/// |       |
/// |       |
/// |     2 |
/// |       |
/// ##########
/// ```
///
/// The calculation will look something like this:
///
/// ```
/// let map = TileMapImpl::new(...);
///
/// let start_position = (3, 3);
/// let end_position = (5, 5);
///
/// let delta = start_position.delta(&end_position);
///
/// assert!(calculate_vertical_slope_in_line_of_sight(
///     &start_position,
///     &end_position,
///     &delta,
///     &get_sign_multiplier(&delta),
///     &map
/// );
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
fn calculate_vertical_slope_in_line_of_sight<T: Tile>(
    start: &impl Position2d,
    end: &impl Position2d,
    delta: &impl Position2d,
    delta_signed: &impl Position2d,
    map: &impl TileMap<T>,
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

        if map.tile_has_collision(&[x, y]) {
            break;
        }
    }

    false
}

/// Determines the sign multiplier of the coordinates for the passed `position` and returns them as an
/// array with a fixed length of `2`. The array contains the sign multiplier for the `x-coordinate` and `y-coordinate`
/// at the first and second index respectively.
///
/// Required for directional corrections during the slope calculations.
///
/// # Arguments
///
/// * `position`: The [Position2d] for which the sign-multipliers should be determined.
///
/// returns: [i32; 2]
///
/// # Examples
///
/// ```
/// let position1 = (30, -4);
/// let position2 = (-30, 4);
///
/// let sign_multiplier1 = get_sign_multiplier(&position1);
/// let sign_multiplier2 = get_sign_multiplier(&position2);
///
/// assert_eq!([1, -1], sign_multiplier1);
/// assert_eq!([-1, 1], sign_multiplier2);
/// ```
///
/// # About
///
/// Authors: [Sebastian Riga](mailto:sebastian.riga.development@gmail.com)
///
/// Since: `0.1.7`
///
fn get_sign_multiplier(position: &impl Position2d) -> [i32; 2] {
    [
        if position.x_coordinate() >= 0 { 1 } else { -1 },
        if position.y_coordinate() >= 0 { 1 } else { -1 },
    ]
}

#[cfg(test)]
mod tests {
    use crate::core::dimension_2d::Dimension2d;
    use crate::ui::game_map::GameMap;
    use crate::ui::tile_map_layout_generator::test::TestTileMapGenerator;

    use super::*;

    #[test]
    fn test_sign_multiplier_evaluation() {
        let position1 = (3, -1);
        let position2 = (-1, 4);
        let position3 = (0, 0);

        assert_eq!([1, -1], get_sign_multiplier(&position1));
        assert_eq!([-1, 1], get_sign_multiplier(&position2));
        assert_eq!([1, 1], get_sign_multiplier(&position3));
    }

    #[test]
    fn test_fov_calculation() {
        let mut map = GameMap::new(&[10, 10], &TestTileMapGenerator);

        let mut fov = Fov::new(8);

        field_of_view(&mut fov, &map.center(), &mut map);

        for position in fov.positions() {
            map.mark_tile_as_seen(position);
            map.mark_tile_as_visible(position);
        }

        for x in 0..map.width - 1 {
            for y in 0..map.height - 1 {
                let position = [x, y];

                if [4, 5, 6].contains(&x) && [4, 5, 6].contains(&y) {
                    assert!(fov.contains(&position));
                    assert!(map.is_tile_seen(&position));
                    assert!(map.is_tile_visible(&position));
                } else {
                    assert!(!fov.contains(&position));
                    assert!(!map.is_tile_seen(&position));
                    assert!(!map.is_tile_visible(&position));
                }
            }
        }
    }
}
