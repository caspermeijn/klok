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

use regex::Regex;
use serde::{Deserialize, Serialize};

use plotters::prelude::*;
use serde_repr::*;
use std::path::PathBuf;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Copy, Clone)]
#[repr(u8)]
enum ChargerState {
    Disabled = 0,
    NoSource = 1,
    Charging = 2,
    ChargeComplete = 3,
    Suspend = 4,
    Fault = 5,
    Other = 6,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct LogEntry {
    #[serde(rename = "ts")]
    timestamp: u64,
    #[serde(rename = "v")]
    voltage: u16,
    #[serde(rename = "t")]
    temperature: u16,
    #[serde(rename = "c")]
    charger: ChargerState,
    #[serde(skip)]
    permyriad: u16,
}

struct Log {
    pub name: String,
    pub entries: Vec<LogEntry>,
}

impl Log {
    fn read_from_file(filename: &PathBuf) -> Result<Log, std::io::Error> {
        let data = std::fs::read_to_string(filename)?;

        let mut entries: Vec<LogEntry> = data
            .lines()
            .filter_map(|line| {
                let re = Regex::new(r"(\{.*})").ok()?;
                re.captures(line).map(|captures| {
                    let json = captures.get(1).unwrap().as_str();
                    let entry: LogEntry = serde_json::from_str(json).unwrap();
                    entry
                })
            })
            .collect();

        entries.sort_by_key(|entry| entry.timestamp);

        Ok(Log {
            name: filename.file_name().unwrap().to_str().unwrap().to_owned(),
            entries,
        }
        .update_permyriad())
    }

    fn remove_non_useful_data(self) -> Self {
        let (first_useful, last_useful) = if self.first().charger == ChargerState::Charging {
            let max_voltage = self
                .entries
                .iter()
                .max_by_key(|entry| entry.voltage)
                .unwrap();

            (
                self.first(),
                self.entries
                    .iter()
                    .find(|entry| (max_voltage.voltage - entry.voltage) < 100)
                    .unwrap(),
            )
        } else {
            (self.first(), self.last())
        };

        Self {
            entries: self
                .entries
                .iter()
                .filter_map(|entry| {
                    if entry.timestamp < first_useful.timestamp
                        || entry.timestamp > last_useful.timestamp
                    {
                        None
                    } else {
                        Some(LogEntry {
                            timestamp: entry.timestamp - first_useful.timestamp,
                            ..*entry
                        })
                    }
                })
                .collect(),
            ..self
        }
        .update_permyriad()
    }

    fn update_permyriad(self) -> Self {
        let first = self.first();
        let last = self.last();
        let diff_timestamp = last.timestamp - first.timestamp;
        Self {
            entries: self
                .entries
                .iter()
                .map(|entry| LogEntry {
                    permyriad: if first.charger == ChargerState::Charging {
                        10000 * (entry.timestamp - first.timestamp) / diff_timestamp
                    } else {
                        10000 * (last.timestamp - entry.timestamp) / diff_timestamp
                    } as u16,
                    ..*entry
                })
                .collect(),
            ..self
        }
    }

    fn first(&self) -> &LogEntry {
        self.entries.first().unwrap()
    }

    fn last(&self) -> &LogEntry {
        self.entries.last().unwrap()
    }

    fn average_temperature(&self) -> u16 {
        let total: usize = self
            .entries
            .iter()
            .map(|entry| entry.temperature as usize)
            .sum();
        (total / self.entries.len()) as u16
    }

    fn get_charger(&self) -> ChargerState {
        let first = self.first();
        assert!(self
            .entries
            .iter()
            .all(|entry| entry.charger == first.charger));
        first.charger
    }
}

fn draw_graph(filename: &str, title: &str, data: Vec<Log>) {
    let first_entry = data.first().unwrap().first();
    let first_reference_list = get_reference_list(data.first().unwrap());
    for log in &data {
        let reference_list = get_reference_list(log);
        assert_eq!(reference_list, first_reference_list);
        for entry in &log.entries {
            assert_eq!(entry.charger, first_entry.charger);
        }
    }

    let root_area = BitMapBackend::new(filename, (1500, 1000)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_left_and_bottom_label_area_size(50)
        .caption(title, ("sans-serif", 40))
        .build_cartesian_2d(
            if first_entry.charger == ChargerState::Charging {
                0..10000
            } else {
                10000..0
            },
            3000..4200,
        )
        .unwrap();

    ctx.configure_mesh()
        .y_desc("Voltage")
        .y_label_formatter(&|y| format!("{:.1}V", *y as f32 / 1000.0))
        .x_desc("State of charge")
        .x_label_formatter(&|x| format!("{}%", x / 100))
        .draw()
        .unwrap();

    for log in &data {
        let style = if log.name.starts_with("low-brightness") {
            &MAGENTA
        } else {
            &GREEN
        };
        ctx.draw_series(LineSeries::new(
            log.entries
                .iter()
                .map(|entry| (entry.permyriad as i32, entry.voltage as i32)),
            style,
        ))
        .unwrap();
    }

    ctx.draw_series(LineSeries::new(
        first_reference_list.iter().map(|reference| {
            (
                reference.state_of_charge as i32 * 100,
                reference.voltage as i32,
            )
        }),
        &BLACK,
    ))
    .unwrap();
}

fn main() {
    // Charging
    let data = std::fs::read_dir("data/charging")
        .unwrap()
        .map(|file| {
            Log::read_from_file(&file.unwrap().path())
                .unwrap()
                .remove_non_useful_data()
        })
        .collect();
    draw_graph("img/charging.png", "Charging", data);

    // Discharging air
    let data = std::fs::read_dir("data/discharging-air")
        .unwrap()
        .map(|file| Log::read_from_file(&file.unwrap().path()).unwrap())
        .collect();

    draw_graph("img/discharging-air.png", "Discharging air", data);

    // Discharging wrist
    let data = std::fs::read_dir("data/discharging-wrist")
        .unwrap()
        .map(|file| Log::read_from_file(&file.unwrap().path()).unwrap())
        .collect();
    draw_graph("img/discharging-wrist.png", "Discharging wrist", data);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ReferenceStateOfCharge {
    voltage: u16,
    state_of_charge: u8,
}

fn get_reference_list(log: &Log) -> Vec<ReferenceStateOfCharge> {
    match log.get_charger() {
        ChargerState::NoSource => {
            let temperature = log.average_temperature();
            if temperature <= 2500 {
                vec![
                    ReferenceStateOfCharge {
                        voltage: 4090,
                        state_of_charge: 100,
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
                ]
            } else if temperature >= 3000 {
                vec![
                    ReferenceStateOfCharge {
                        voltage: 4090,
                        state_of_charge: 100,
                    },
                    ReferenceStateOfCharge {
                        voltage: 3850,
                        state_of_charge: 75,
                    },
                    ReferenceStateOfCharge {
                        voltage: 3720,
                        state_of_charge: 55,
                    },
                    ReferenceStateOfCharge {
                        voltage: 3630,
                        state_of_charge: 25,
                    },
                    ReferenceStateOfCharge {
                        voltage: 3550,
                        state_of_charge: 6,
                    },
                    ReferenceStateOfCharge {
                        voltage: 3300,
                        state_of_charge: 0,
                    },
                ]
            } else {
                unimplemented!()
            }
        }
        ChargerState::Charging => {
            vec![
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
            ]
        }
        _ => unimplemented!(),
    }
}
