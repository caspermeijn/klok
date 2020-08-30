<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (C) 2020 Casper Meijn <casper@meijn.net>

This work is licensed under the Creative Commons Attribution-ShareAlike 4.0 International License. 
To view a copy of this license, visit http://creativecommons.org/licenses/by-sa/4.0/ or 
  send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

Bluetooth device information
============================

This firmware provides a BLE (Bluetooth Low Energy) DIS (Device Information) service
so that other devices can recognize the firmware. The full specification of the 
service can be found on the [Bluetooth website](https://www.bluetooth.com/specifications/gatt/).

## Service Characteristics

The following values are used:

Characteristic name | Value | Comment
--- | --- | ---
Manufacturer Name | "PINE64" | Manufacturer name as used on www.pine64.org
Model Number | "PineTime" | Hardware name as used on www.pine64.org
Hardware Revision | "1.0a" | Hardware version as used on [schamatic](http://files.pine64.org/doc/PineTime/PineTime%20Schematic-V1.0a-20191103.pdf)
Software Revision | "Klok" | Firmware name
Firmware Revision | "0.2.0" | Firmware version number as used in release
