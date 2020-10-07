<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (C) 2020 Casper Meijn <casper@meijn.net>

This work is licensed under the Creative Commons Attribution-ShareAlike 4.0 International License. 
To view a copy of this license, visit http://creativecommons.org/licenses/by-sa/4.0/ or 
  send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

Upgrade
-------
This document is for when Klok is already installed on your PineTime hardware and you want to upgrade 
to a newer version. You could to use the [installation](docs/installation.md) instructions 
with the new image or use these instructions to upgrade wireless via Bluetooth Low Energy (BLE).


### nRF Connect
This method is easiest for most users, but requires an Android phone.

Instructions:
- Install [nRF Connect](https://play.google.com/store/apps/details?id=no.nordicsemi.android.mcp) on a Android smartphone.
- Download [klok_dfu.bin](https://gitlab.com/caspermeijn/klok/-/jobs/artifacts/master/raw/dfu/klok_dfu.bin?job=package) to the phone
- Open "nRF Connect" app
- Choose "connect" for your PineTime device
- In the right-upper corner choose the "DFU" icon
- Choose the downloaded file
- Select "Test and Confirm" mode
- Wait for the upgrade to complete
