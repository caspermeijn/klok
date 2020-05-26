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

extern "C" {
    fn sysinit_start();
    fn sysinit_app();
    fn sysinit_end();
    fn hal_gpio_init_out(pin: i32, val: i32) -> i32;
    fn hal_gpio_toggle(pin: i32);
    fn os_time_delay(osticks: u32);
}

extern crate panic_halt;

const OS_TICKS_PER_SEC: u32 = 128;

const LCD_BACKLIGHT_HIGH_PIN: i32 = 23;
const LED_BLINK_PIN: i32 = LCD_BACKLIGHT_HIGH_PIN;

#[no_mangle]
pub extern "C" fn main() {
    /* Initialize all packages. */
    unsafe { sysinit_start(); }
    unsafe { sysinit_app(); }
    unsafe { sysinit_end(); }

    unsafe { hal_gpio_init_out(LED_BLINK_PIN, 1); }

    loop {
        /* Wait one second */
        unsafe { os_time_delay(OS_TICKS_PER_SEC); }

        /* Toggle the LED */
        unsafe { hal_gpio_toggle(LED_BLINK_PIN); }
    }
}
