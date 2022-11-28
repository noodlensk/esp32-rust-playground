#![deny(clippy::all)]
extern crate core;

mod faces;
mod tamagotchi;
mod wifi;

use crate::tamagotchi::Tamagothci;
use std::{thread, time};

fn main() {
    esp_idf_sys::link_patches();

    let mut t = Tamagothci::new();

    let mut wifi = wifi::WiFi::new();

    loop {
        t.draw();
        wifi.next_channel();
        thread::sleep(time::Duration::from_secs(10));
    }
}
