<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (C) 2020 Casper Meijn <casper@meijn.net>

This work is licensed under the Creative Commons Attribution-ShareAlike 4.0 International License. 
To view a copy of this license, visit http://creativecommons.org/licenses/by-sa/4.0/ or 
  send a letter to Creative Commons, PO Box 1866, Mountain View, CA 94042, USA.
-->

Installation
------------
There are multiple ways of installing this firmware onto the PineTime hardware:

### Upgrade from other firmware
This method can be used if you have already installed a compatible firmware. This method is tested with the following 
firmwares:
- [InfiniTime](https://github.com/JF002/InfiniTime) 1.0.0

Instructions:
- Install [nRF Connect](https://play.google.com/store/apps/details?id=no.nordicsemi.android.mcp) on a Android smartphone.
- Download [klok_dfu.zip](https://gitlab.com/caspermeijn/klok/-/jobs/artifacts/master/raw/dfu/klok_dfu.zip?job=package) to the phone
- Open "nRF Connect" app
- Choose "connect" for your PineTime device
- In the right-upper corner choose the "DFU" icon
- Choose "Distribution ZIP"
- Choose the downloaded file
- Wait for the upgrade to complete
- Connect to the device again
- Click "SMP Service"
- Click the "single arrow up" icon
- Select command "Image / Confirm"
- Click "Send"

### PineTime Updater
This method is most suitable for first time use of the PineTime, especially when the Flash ROM Protection is still 
enabled. Someone else in the community created a tool for easily flashing firmwares. 

- Install [pinetime-updater](https://github.com/lupyuen/pinetime-updater) by following the [instructions](https://github.com/lupyuen/pinetime-updater#how-to-run).
- Remove the Flash ROM protection (if you have not done this yet) by following the [instructions](https://github.com/lupyuen/pinetime-updater#remove-flash-rom-protection).
- Flash the "Latest Bootloader" via the menu
- Flash Klok by choosing "Download from URL" in the menu. Enter this [URL](https://gitlab.com/caspermeijn/klok/-/jobs/artifacts/master/raw/klok.img?job=package)
    (try right-click and copy link location). Use the default flash address.

### Build and Load
This method downloads the sources, build the firmware locally and uses the build tools flash the firmware. This is the
best method if you plan to do development on Klok. If you are just a user, then this is not the method for you.

You need the following dependencies installed:

- [newt](https://mynewt.apache.org/latest/get_started/index.html) 
- arm-none-eabi-gcc (on fedora: `sudo dnf install arm-none-eabi-gcc arm-none-eabi-newlib`)
- rust-lang nightly with cross-compile target (for example `rustup target add thumbv7m-none-eabi`)

```bash
# Navigate to your source code directory
cd ~/src
# Download the firmware code
git clone https://gitlab.com/caspermeijn/klok.git
cd klok
# Download all depencenies
newt upgrade
# Build the bootloader
newt build boot-pinetime
# Flash the bootloader
newt load boot-pinetime
# Build the application
newt create-image klok-pinetime 0
# Flash the application
newt load klok-pinetime
```  
