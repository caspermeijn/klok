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

extern crate panic_semihosting;

extern crate mynewt_core_hw_hal as hal;

use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};
use embedded_hal::blocking::delay::DelayMs;
use st7789::{Orientation, ST7789};

use core::fmt::Write;
use heapless::consts::*;
use heapless::String;
use mynewt_core_kernel_os::callout::Callout;
use mynewt_core_kernel_os::queue::EventQueue;
use mynewt_core_kernel_os::task::Task;
use mynewt_core_kernel_os::time::{Delay, TimeChangeListener, TimeOfDay};
use mynewt_core_mgmt_imgmgr::ImageVersion;
use mynewt_nimble_host::advertiser::BleAdvertiser;
use watchface::Watchface;

extern "C" {
    fn sysinit_start();
    fn sysinit_app();
    fn sysinit_end();

    fn battery_measurement_init();
}

struct TimeOfDayProvider {}

impl watchface::TimeProvider for TimeOfDayProvider {
    fn get_time(&self) -> String<U8> {
        let time = TimeOfDay::getTimeOfDay().unwrap();

        let mut text = String::new();
        write!(
            &mut text,
            "{:02}:{:02}",
            time.hours_local(),
            time.minutes_local(),
        )
        .unwrap();
        text
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

    let mut display_spi = unsafe { BSP.display_spi.take().unwrap() };
    let mut display_data_command = unsafe { BSP.display_data_command.take().unwrap() };
    let mut display_reset = unsafe { BSP.display_reset.take().unwrap() };

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
                unsafe {
                    DRAW_CALLOUT.reset(delay_seconds as u32 * 1000);
                }
            },
            &mut DRAW_EVENTQ,
        );
    }
    unsafe { DRAW_CALLOUT.reset(1000) };

    loop {
        unsafe { DRAW_EVENTQ.run() };
    }
}

static mut BSP: mynewt_pinetime_bsp::Bsp = mynewt_pinetime_bsp::Bsp::new();
static mut TASK: Task = Task::new();
static mut BACKLIGHT_CALLOUT: Callout = Callout::new();
static mut TIME_CHANGE_LISTENER: TimeChangeListener = TimeChangeListener::new();
static mut VERSION_STRING: Option<String<U12>> = None;

#[no_mangle]
pub extern "C" fn main() {
    /* Initialize all packages. */
    unsafe {
        sysinit_start();
        sysinit_app();
        sysinit_end();
    }

    let version = mynewt_core_mgmt_imgmgr::ImageVersion::get_current().unwrap();
    let mut version_string: String<U12> = version.into();
    version_string.push_str("\0").unwrap();
    unsafe {
        VERSION_STRING = Some(version_string);
    }
    mynewt_nimble_host_services::device_information::set_firmware_revision(unsafe {
        VERSION_STRING.as_ref().unwrap()
    });

    unsafe { battery_measurement_init(); }

    mynewt_core_sys_config::conf_load();

    mynewt_core_sys_reboot::reboot_start();

    unsafe {
        BSP.init();
    }
    let mut delay = Delay {};

    let mut backlight_high = unsafe { BSP.backlight_high.take().unwrap() };
    backlight_high.write(hal::gpio::PinState::High);

    unsafe {
        TASK.init("draw", draw_task, 200);
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
                unsafe { BACKLIGHT_CALLOUT.reset(1000) };
            })
        };
        unsafe { BACKLIGHT_CALLOUT.reset(1000) };
    }

    BleAdvertiser::start();

    mynewt_core_kernel_os::queue::loop_default_queue();
}
