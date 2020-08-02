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
use mynewt_core_kernel_os::time::{TimeOfDay, Delay};
use heapless::String;
use heapless::consts::*;
use core::fmt::Write;
use mynewt_core_kernel_os::task::Task;
use mynewt_core_kernel_os::callout::Callout;

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

fn draw_task() {
    let mut spi = unsafe { BSP.spi.take().unwrap() };
    let mut display_data_command = unsafe { BSP.display_data_command.take().unwrap() };
    let mut display_chip_select = unsafe { BSP.display_chip_select.take().unwrap() };
    let mut display_reset = unsafe { BSP.display_reset.take().unwrap() };

    // display interface abstraction from SPI and DC
    let di = SPIInterface::new(spi, display_data_command, display_chip_select);

    // create driver
    let mut display = ST7789::new(di, display_reset, 240, 240);

    let mut delay = Delay {};

    // initialize
    display.init(&mut delay).unwrap();
    // set default orientation
    display.set_orientation(Orientation::Portrait).unwrap();

    // draw two circles on black background
    display.clear(Rgb565::BLACK).unwrap();

    let now_provider = TimeOfDayProvider {};
    let battery_provider = StubBatteryProvider {};

    let watchface = Watchface::new(now_provider, battery_provider);

    loop {
        watchface.draw(&mut display).unwrap();

        delay.delay_ms(100);
    }
}

static mut BSP: mynewt_pinetime_bsp::Bsp = mynewt_pinetime_bsp::Bsp::new();
static mut TASK: Task = Task::new();
static mut BACKLIGHT_CALLOUT:Callout = Callout::new();

#[no_mangle]
pub extern "C" fn main() {
    /* Initialize all packages. */
    unsafe { sysinit_start(); }
    unsafe { sysinit_app(); }
    unsafe { sysinit_end(); }

    unsafe { BSP.init(); }
    let mut delay = Delay {};

    let mut backlight_high = unsafe { BSP.backlight_high.take().unwrap() };
    backlight_high.write(hal::gpio::PinState::Low);

    unsafe { TASK.init("draw", draw_task, 200); }

    unsafe { BACKLIGHT_CALLOUT.init_default_queue(
        move || {
            backlight_high.toggle();
            unsafe { BACKLIGHT_CALLOUT.reset(1000) };
        }
    )};
    unsafe { BACKLIGHT_CALLOUT.reset(1000) };

    mynewt_core_kernel_os::queue::loop_default_queue();
}
