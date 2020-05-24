/* Copyright (C) 2020 Casper Meijn <casper@meijn.net>
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

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

pub trait Center<D: DrawTarget<Rgb565>> {
    fn center(&mut self, display: &D) -> Self;
}

impl<D,T> Center<D> for T
    where D: DrawTarget<Rgb565>,
          T: embedded_graphics::transform::Transform + embedded_graphics::geometry::Dimensions,
{
    fn center(&mut self, display: &D) -> Self {
        let display_size = display.size();
        let drawable_size = self.size();
        let center_x = ((display_size.width - drawable_size.width) / 2) as i32;
        let center_y = ((display_size.height - drawable_size.height) / 2) as i32;
        let center_point = Point::new(center_x, center_y);
        self.translate(center_point)
    }
}

pub trait TopRight<D: DrawTarget<Rgb565>> {
    fn translate_to_top_right(&mut self, display: &D) -> Self;
}

impl<D,T> TopRight<D> for T
    where D: DrawTarget<Rgb565>,
          T: embedded_graphics::transform::Transform + embedded_graphics::geometry::Dimensions,
{
    fn translate_to_top_right(&mut self, display: &D) -> Self {
        let display_size = display.size();
        let drawable_size = self.size();
        let top_right_x = (display_size.width - drawable_size.width - 20) as i32;
        let top_right_y = 20;
        let top_right_point = Point::new(top_right_x, top_right_y);
        self.translate(top_right_point)
    }
}
