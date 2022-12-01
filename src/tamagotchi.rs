mod display;
mod faces;
mod wifi;
mod rtc;

use std::{thread, time};

use esp_idf_hal::gpio::{Gpio13, Output, PinDriver};
use esp_idf_hal::peripherals::Peripherals;

use crate::tamagotchi::wifi::WiFi;
use crate::tamagotchi::display::EInk;


pub struct Tamagothci<'a> {
    display: EInk<'a>,
    vib_motor_pin: PinDriver<'a, Gpio13, Output>,
    wifi: WiFi,
    rtc: rtc::Rtc<'a>,
}

impl<'a> Tamagothci<'a> {
    pub fn new() -> Result<Self, String> {
        let peripherals = match Peripherals::take() {
            Some(peripherals) => peripherals,
            None => return Err(String::from("empty peripherals"))
        };

        let rtc = match rtc::Rtc::new(peripherals.i2c0, peripherals.pins.gpio21, peripherals.pins.gpio22) {
            Ok(rtc) => rtc,
            Err(error) => return Err(error)
        };

        Ok(Tamagothci {
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
            rtc,
        })
    }

    pub fn redraw(&mut self) -> Result<(), String> {
        self.wifi.next_channel();

        let known_networks = WiFi::known_networks();

        for (key, network) in known_networks.iter() {
            println!("{} {}", key, network.ssid);
        }

        let curr_time = match self.rtc.current_time() {
            Ok(time) => time,
            Err(error) => return Err(error),
        };

        match self.display.draw(curr_time) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }

    pub fn vibrate_short(&mut self) {
        self.vib_motor_pin.set_high().unwrap();
        thread::sleep(time::Duration::from_millis(50));
        self.vib_motor_pin.set_low().unwrap();
    }
}
