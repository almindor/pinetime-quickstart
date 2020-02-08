# pinetime-quickstart

PineTime "smartwatch" quickstart repo for Rust development with HAL

This is a work in progress. As of right now this setup has been tested with the PineTime watch using a JLink Mini-Edu probe and the Segger JLinkGDBRemote software. No other probe/setup has been tested yet.

## JLink hardware setup

Connect the SWD to your JLink probe and connect the probe to the computer. A breakout board might be needed so you can use standard breakout pins.

## JLink software setup

### Requirements

* Segger JLink software
* `arm-none-eabi-gdb`

### Setup

**WARNING**
Flashing the device removes the closed-source OS that comes with PineTime. I'm not aware of any way to get it back (there might be a hex file somewhere).

You should run `JLinkExe` for first time so you can unlock the "locked flash/ram" on the PineTime. A prompt should pop up, if not check if your Segger software is up to date. Once the re-flash is done you can continue using `JLinkGDBServer` as shown below:

1. Run `JLinkGDBServer -device NRF52832_XXAA -if SWD -speed 4000 -port 3333 -nogui
1a. You might get a pop up if using JLink Mini-Edu that needs to be accepted once per day
2. Once `JLinkGDBServer` is connect you should be good to go and just use `cargo run` to execute the main program. Use the examples as required.
