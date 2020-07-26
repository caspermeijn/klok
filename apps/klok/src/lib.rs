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

#![no_std]

extern crate panic_semihosting;

extern crate mynewt_core_hw_hal as hal;

use display_interface_spi::SPIInterface;
use st7789::{ST7789, Orientation};
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_hal::blocking::delay::DelayMs;

use watchface::Watchface;
use mynewt_core_kernel_os::time::TimeOfDay;
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;

extern "C" {
    fn sysinit_start();
    fn sysinit_app();
    fn sysinit_end();
}

struct TimeOfDayProvider {
}

impl watchface::TimeProvider for TimeOfDayProvider {
    fn get_time(&self) -> String<U8> {
        let time = TimeOfDay::getTimeOfDay().unwrap();

        let mut text = String::new();
        write!(&mut text, "{:02}:{:02}:{:02}", time.hours(), time.minutes(), time.seconds()).unwrap();
        text
    }
}

struct StubBatteryProvider {}

impl watchface::BatteryProvider for StubBatteryProvider {
    fn get_state_of_charge(&self) -> f32 {
        0.5
    }
}

#[no_mangle]
pub extern "C" fn main() {
    /* Initialize all packages. */
    unsafe { sysinit_start(); }
    unsafe { sysinit_app(); }
    unsafe { sysinit_end(); }

    let mut bsp = mynewt_pinetime_bsp::Bsp::new();

    bsp.backlight_high.write(hal::gpio::PinState::Low);

    // display interface abstraction from SPI and DC
    let di = SPIInterface::new(bsp.spi, bsp.display_data_command, bsp.display_chip_select);

    // create driver
    let mut display = ST7789::new(di, bsp.display_reset, 240, 240);

    // initialize
    display.init(&mut bsp.delay).unwrap();
    // set default orientation
    display.set_orientation(Orientation::Portrait).unwrap();

    // draw two circles on black background
    display.clear(Rgb565::BLACK).unwrap();

    let now_provider = TimeOfDayProvider {};
    let battery_provider = StubBatteryProvider {};

    let watchface = Watchface::new(now_provider, battery_provider);

    loop {
        watchface.draw(&mut display).unwrap();

        bsp.delay.delay_ms(100);
    }
}
