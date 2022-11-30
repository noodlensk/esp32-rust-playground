#![feature(local_key_cell_methods)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
extern crate core;

mod faces;
mod tamagotchi;
mod wifi;

use crate::tamagotchi::Tamagothci;
use crate::wifi::WiFi;
use std::{thread, time};

fn main() {
    esp_idf_sys::link_patches();

    let mut t = Tamagothci::new();

    let mut wifi = WiFi::new();

    loop {
        t.draw();
        wifi.next_channel();
        let known_networks = WiFi::known_networks();

        for (key, network) in known_networks.iter() {
            println!("{} {}", key, network.ssid);
        }

        // t.vibrate_short();
        thread::sleep(time::Duration::from_secs(10));
    }
}
