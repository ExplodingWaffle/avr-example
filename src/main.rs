#![no_std]
#![no_main]
#![allow(unused_imports)]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_leonardo::{hal::{self, usb::UsbBus}, prelude::*};
use class_prelude::*;
use usbd_serial::*;
use usb_device::*;
use usb_device::bus::*;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let usb_bus = UsbBus::new(dp.USB_DEVICE, dp.PLL);
    let usb_bus_alloc = UsbBusAllocator::<UsbBus>::new(usb_bus);

    let mut serial = SerialPort::new(&usb_bus_alloc);

    let mut usb_dev = usb_device::prelude::UsbDeviceBuilder::new(&usb_bus_alloc, usb_device::prelude::UsbVidPid(0x16c0, 0x27dd))
    .manufacturer("Fake company")
    .product("Serial port")
    .serial_number("TEST")
    .device_class(USB_CLASS_CDC)
    .build();

    let pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
    let led0 = pins.d0;
    let led1 = pins.d1;
    let led2 = pins.d2;
    let led3 = pins.d3;
    let led4 = pins.d4;
    let led5 = pins.d5;
    let led6 = pins.d6;
    let led7 = pins.d7;

    let leds = &mut [
        led0.into_output(&pins.ddr).downgrade(),
        led1.into_output(&pins.ddr).downgrade(), 
        led2.into_output(&pins.ddr).downgrade(), 
        led3.into_output(&pins.ddr).downgrade(), 
        led4.into_output(&pins.ddr).downgrade(), 
        led5.into_output(&pins.ddr).downgrade(), 
        led6.into_output(&pins.ddr).downgrade(), 
        led7.into_output(&pins.ddr).downgrade()
        ];

    usb_dev.force_reset().ok();

    match usb_dev.state() {
        device::UsbDeviceState::Default => {leds[0].set_high().ok();}
        device::UsbDeviceState::Addressed => {leds[1].set_high().ok();}
        device::UsbDeviceState::Configured => {leds[2].set_high().ok();}
        device::UsbDeviceState::Suspend => {leds[3].set_high().ok();}
    };

    loop {

        leds[5].toggle().ok();

        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        leds[6].toggle().ok();

    }

//    loop {
//        for n in 0..leds.len() {
//            leds[n].toggle().ok();
//            arduino_leonardo::delay_ms(40);
//        }
//    };
}