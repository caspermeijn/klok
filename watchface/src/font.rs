/* SPDX-FileCopyrightText: © 2020 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use embedded_graphics::prelude::*;

const CHARS_PER_ROW: u32 = 16;

#[derive(Debug, Copy, Clone)]
pub struct NumbersFont {}
impl Font for NumbersFont {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/numbers.raw");
    const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
    const CHARACTER_SIZE: Size = Size::new(40, 52);
    fn char_offset(c: char) -> u32 {
        match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            ':' => 10,
            _ => 15,
        }
    }
}
