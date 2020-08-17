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

use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

use chrono::Timelike;
use heapless::consts::*;
use heapless::String;
use std::thread;
use std::time::Duration;
use watchface::BatteryProvider;
use watchface::TimeProvider;
use watchface::Watchface;

struct NowProvider {}

impl TimeProvider for NowProvider {
    fn get_time(&self) -> String<U8> {
        heapless::String::from(chrono::Local::now().format("%H:%M").to_string().as_ref())
    }
}

struct StubBatteryProvider {}

impl BatteryProvider for StubBatteryProvider {
    fn get_state_of_charge(&self) -> f32 {
        chrono::Local::now().second() as f32 / 59.0
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));

    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Klok watchface", &output_settings);

    let now_provider = NowProvider {};
    let battery_provider = StubBatteryProvider {};

    let watchface = Watchface::new(now_provider, Some(battery_provider));

    'running: loop {
        watchface.draw(&mut display)?;
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
}
