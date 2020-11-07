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

#![feature(const_fn)]
#![no_std]

extern crate alloc;
extern crate panic_semihosting;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use st7789::{Orientation, ST7789};

use alloc::format;
use alloc::string::String;
use mynewt::core::hw::bsp::pinetime::Bsp;
use mynewt::core::hw::hal::gpio::PinState;
use mynewt::core::kernel::os::callout::Callout;
use mynewt::core::kernel::os::queue::EventQueue;
use mynewt::core::kernel::os::task::Task;
use mynewt::core::kernel::os::time::{Delay, TimeChangeListener, TimeOfDay};
use mynewt::core::mgmt::imgmgr::ImageVersion;
use mynewt::nimble::host::advertiser::BleAdvertiser;
use watchface::Watchface;

extern "C" {
    fn sysinit_start();
    fn sysinit_app();
    fn sysinit_end();

    fn battery_measurement_init();
}

struct TimeOfDayProvider {}

impl watchface::TimeProvider for TimeOfDayProvider {
    fn get_time(&self) -> String {
        let time = TimeOfDay::getTimeOfDay().unwrap();

        format!("{:02}:{:02}", time.hours_local(), time.minutes_local(),)
    }
}

struct StubBatteryProvider {}

impl watchface::BatteryProvider for StubBatteryProvider {
    fn get_state_of_charge(&self) -> f32 {
        0.5
    }
}

static mut DRAW_EVENTQ: EventQueue = EventQueue::new();
static mut DRAW_CALLOUT: Callout = Callout::new();

fn draw_task() {
    unsafe { DRAW_EVENTQ.init() };

    let bsp = unsafe { Bsp::steal() };
    let display_spi = bsp.display_spi;
    let display_data_command = bsp.display_data_command;
    let display_reset = bsp.display_reset;

    // display interface abstraction from SPI and DC
    let di = SPIInterfaceNoCS::new(display_spi, display_data_command);

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

    let watchface: Watchface<_, StubBatteryProvider> = Watchface::new(now_provider, None);

    unsafe {
        DRAW_CALLOUT.init(
            move || {
                watchface.draw(&mut display).unwrap();

                let time = TimeOfDay::getTimeOfDay().unwrap();
                let delay_seconds = 60 - time.seconds();
                DRAW_CALLOUT.reset(delay_seconds as u32 * 1000);
            },
            &mut DRAW_EVENTQ,
        );
    }
    unsafe { DRAW_CALLOUT.reset(1000) };

    loop {
        unsafe { DRAW_EVENTQ.run() };
    }
}

static mut TASK: Task = Task::new();
static mut BACKLIGHT_CALLOUT: Callout = Callout::new();
static mut TIME_CHANGE_LISTENER: TimeChangeListener = TimeChangeListener::new();

#[no_mangle]
pub extern "C" fn main() {
    /* Initialize all packages. */
    unsafe {
        sysinit_start();
        sysinit_app();
        sysinit_end();
    }

    let bsp = Bsp::take().unwrap();

    let version = ImageVersion::get_current().unwrap();
    mynewt::nimble::host::services::device_information::set_firmware_revision(version.into());

    unsafe {
        battery_measurement_init();
    }

    mynewt::core::sys::config::conf_load();

    mynewt::core::sys::reboot::reboot_start();

    let mut backlight_high = bsp.backlight_high;
    backlight_high.write(PinState::High);

    unsafe {
        TASK.init("draw", draw_task, 200).unwrap();
    }

    unsafe {
        TIME_CHANGE_LISTENER.register(move || {
            DRAW_CALLOUT.reset(0);
        })
    }

    if false {
        unsafe {
            BACKLIGHT_CALLOUT.init_default_queue(move || {
                backlight_high.toggle();
                BACKLIGHT_CALLOUT.reset(1000);
            });
            BACKLIGHT_CALLOUT.reset(1000);
        }
    }

    BleAdvertiser::start();

    mynewt::core::kernel::os::queue::loop_default_queue();
}
