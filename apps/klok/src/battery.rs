/* Copyright (C) 2021 Casper Meijn <casper@meijn.net>
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

use mynewt::core::hw::battery::{BatteryStatus, Property};
use mynewt::core::hw::charge_control;
use mynewt::core::hw::charge_control::{ChargeControl, Status};
use watchface::battery::{ChargerState, StateOfCharge};

pub fn try_get_state_of_charge() -> Result<StateOfCharge, ()> {
    let mut battery = mynewt::core::hw::battery::Battery::get_by_number(0)?;
    if let Ok(property) =
        battery.find_property(mynewt::core::hw::battery::PropertyType::StateOfCharge)
    {
        try_get_state_of_charge_from_soc_property(property)
    } else if let Ok(property) =
        battery.find_property(mynewt::core::hw::battery::PropertyType::VoltageNow)
    {
        try_get_state_of_charge_from_voltage(property)
    } else {
        Err(())
    }
}

fn try_get_state_of_charge_from_soc_property(
    property_state_of_charge: Property,
) -> Result<StateOfCharge, ()> {
    if let Some(mynewt::core::hw::battery::PropertyValue::StateOfCharge(state_of_charge)) =
        property_state_of_charge.get_value()
    {
        Ok(StateOfCharge::from_percentage(state_of_charge))
    } else {
        Err(())
    }
}

fn try_get_state_of_charge_from_voltage(property_voltage: Property) -> Result<StateOfCharge, ()> {
    if let Some(mynewt::core::hw::battery::PropertyValue::Voltage(voltage_mv)) =
        property_voltage.get_value()
    {
        Ok(convert_voltage_to_percentage(voltage_mv as u16))
    } else {
        Err(())
    }
}

pub fn try_get_charger_status() -> Result<ChargerState, ()> {
    let mut battery = mynewt::core::hw::battery::Battery::get_by_number(0)?;
    if let Ok(property_status) =
        battery.find_property(mynewt::core::hw::battery::PropertyType::Status)
    {
        if let Some(mynewt::core::hw::battery::PropertyValue::Status(status)) =
            property_status.get_value()
        {
            Ok(match status {
                BatteryStatus::Unknown => ChargerState::Discharging,
                BatteryStatus::Charging => ChargerState::Charging,
                BatteryStatus::Discharging => ChargerState::Discharging,
                BatteryStatus::NotCharging => ChargerState::Discharging,
                BatteryStatus::Full => ChargerState::Full,
            })
        } else {
            Err(())
        }
    } else {
        let mut charger = ChargeControl::find_first_by_type(charge_control::Type::Status)?;

        let status = charger.read_status_blocking();
        Ok(match status {
            Status::Disabled => ChargerState::Discharging,
            Status::NoSource => ChargerState::Discharging,
            Status::Charging => ChargerState::Charging,
            Status::ChargeComplete => ChargerState::Full,
            Status::Suspend => ChargerState::Discharging,
            Status::Fault => ChargerState::Discharging,
            Status::Other => ChargerState::Discharging,
        })
    }
}

fn convert_voltage_to_percentage(voltage_mv: u16) -> StateOfCharge {
    let charging = match try_get_charger_status() {
        Ok(ChargerState::Charging) => true,
        Ok(ChargerState::Full) => true,
        _ => false,
    };
    let reference = if charging {
        &CHARGING_REFERENCE[..]
    } else {
        &DISCHARGING_REFERENCE[..]
    };
    let mut high_point = None;
    let mut low_point = None;
    for ref_point in reference {
        if ref_point.voltage > voltage_mv {
            high_point = Some(ref_point)
        }
        if ref_point.voltage < voltage_mv {
            low_point = Some(ref_point);
            break;
        }
    }
    if let Some(high_point) = high_point {
        if let Some(low_point) = low_point {
            let a = (high_point.state_of_charge as f32 - low_point.state_of_charge as f32)
                / (high_point.voltage as f32 - low_point.voltage as f32);
            let b = high_point.state_of_charge as f32 - a * high_point.voltage as f32;

            let percent_remaining = a * voltage_mv as f32 + b;

            StateOfCharge::from_percentage(percent_remaining as u8)
        } else {
            StateOfCharge::from_percentage(0)
        }
    } else {
        StateOfCharge::from_percentage(100)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ReferenceStateOfCharge {
    voltage: u16,
    state_of_charge: u8,
}

const DISCHARGING_REFERENCE: [ReferenceStateOfCharge; 6] = [
    ReferenceStateOfCharge {
        voltage: 4090,
        state_of_charge: 100,
    },
    ReferenceStateOfCharge {
        voltage: 3850,
        state_of_charge: 75,
    },
    ReferenceStateOfCharge {
        voltage: 3730,
        state_of_charge: 55,
    },
    ReferenceStateOfCharge {
        voltage: 3650,
        state_of_charge: 25,
    },
    ReferenceStateOfCharge {
        voltage: 3550,
        state_of_charge: 5,
    },
    ReferenceStateOfCharge {
        voltage: 3300,
        state_of_charge: 0,
    },
];

const CHARGING_REFERENCE: [ReferenceStateOfCharge; 4] = [
    ReferenceStateOfCharge {
        voltage: 4090,
        state_of_charge: 100,
    },
    ReferenceStateOfCharge {
        voltage: 3920,
        state_of_charge: 85,
    },
    ReferenceStateOfCharge {
        voltage: 3670,
        state_of_charge: 5,
    },
    ReferenceStateOfCharge {
        voltage: 3400,
        state_of_charge: 0,
    },
];
