mod display;
mod faces;
mod wifi;

use std::{thread, time};

use esp_idf_hal::gpio::{Gpio13, Output, PinDriver};
use esp_idf_hal::peripherals::Peripherals;

use crate::tamagotchi::wifi::WiFi;
use crate::tamagotchi::display::EInk;


pub struct Tamagothci<'a> {
    display: EInk<'a>,
    vib_motor_pin: PinDriver<'a, Gpio13, Output>,
    wifi: WiFi,
}

impl<'a> Tamagothci<'a> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap();

        Tamagothci {
            display: EInk::new(
                peripherals.spi2,
                peripherals.pins.gpio23,
                peripherals.pins.gpio18,
                peripherals.pins.gpio5,
                peripherals.pins.gpio9,
                peripherals.pins.gpio10,
                peripherals.pins.gpio19,
            ),
            vib_motor_pin: PinDriver::output(peripherals.pins.gpio13).unwrap(),
            wifi: WiFi::new(),
        }
    }

    pub fn redraw(&mut self) {
        self.wifi.next_channel();

        let known_networks = WiFi::known_networks();

        for (key, network) in known_networks.iter() {
            println!("{} {}", key, network.ssid);
        }

        self.display.draw_random_face().unwrap();
        self.vibrate_short();
    }

    pub fn vibrate_short(&mut self) {
        self.vib_motor_pin.set_high().unwrap();
        thread::sleep(time::Duration::from_millis(50));
        self.vib_motor_pin.set_low().unwrap();
    }
}
