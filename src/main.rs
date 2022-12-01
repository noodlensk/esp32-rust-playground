#![feature(local_key_cell_methods)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
extern crate core;

mod tamagotchi;

use crate::tamagotchi::Tamagothci;
use std::{thread, time};

fn main() {
    esp_idf_sys::link_patches();

    let mut t = Tamagothci::new().unwrap();

    loop {
        t.redraw().unwrap();

        thread::sleep(time::Duration::from_secs(10));
    }
}
