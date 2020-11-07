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

use crate::battery_icon::*;
use crate::drawable_utils::*;
use crate::font::NumbersFont;
use alloc::string::String;
use embedded_graphics::style::TextStyleBuilder;
use embedded_graphics::{fonts::Text, pixelcolor::Rgb565, prelude::*};

pub trait BatteryProvider {
    fn get_state_of_charge(&self) -> f32;
}

pub trait TimeProvider {
    fn get_time(&self) -> String;
}

pub struct Watchface<TP, BP>
where
    TP: TimeProvider,
    BP: BatteryProvider,
{
    time_provider: TP,
    battery_provider: Option<BP>,
}

impl<TP, BP> Watchface<TP, BP>
where
    TP: TimeProvider,
    BP: BatteryProvider,
{
    pub fn new(time_provider: TP, battery_provider: Option<BP>) -> Watchface<TP, BP> {
        Watchface {
            time_provider,
            battery_provider,
        }
    }

    pub fn draw<D: DrawTarget<Rgb565>>(
        &self,
        display: &mut D,
    ) -> core::result::Result<(), D::Error> {
        let time_text_style = TextStyleBuilder::new(NumbersFont {})
            .text_color(Rgb565::WHITE)
            .background_color(Rgb565::BLACK)
            .build();

        let time = self.time_provider.get_time();
        Text::new(&time, Point::zero())
            .into_styled(time_text_style)
            .center(display)
            .draw(display)?;

        if let Some(battery_provider) = &self.battery_provider {
            BatteryIcon {
                top_left: Point::new(0, 0),
                bottom_right: Point::new(10, 20),
                bg_color: Rgb565::BLACK,
                fg_color: Rgb565::WHITE,
                empty_color: Rgb565::RED,
                full_color: Rgb565::GREEN,
                state_of_charge: battery_provider.get_state_of_charge(),
            }
            .translate_to_top_right(display)
            .draw(display)?;
        }

        Ok(())
    }
}
