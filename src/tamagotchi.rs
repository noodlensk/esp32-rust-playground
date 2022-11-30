use crate::faces;

use std::{thread, time};

use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;
use embedded_graphics::Drawable;

use epd_waveshare::epd1in54_v2::{Display1in54, Epd1in54};
use epd_waveshare::graphics::Display;
use epd_waveshare::prelude::{RefreshLut, WaveshareDisplay};

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Gpio10, Gpio13, Gpio19, Gpio9, Input, Output, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::spi::{Dma, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::{delay, gpio, spi};

type DisplayDevice<'a> = Epd1in54<
    SpiDeviceDriver<'a, SpiDriver<'a>>,
    PinDriver<'a, Gpio19, Input>,
    PinDriver<'a, Gpio10, Output>,
    PinDriver<'a, Gpio9, Output>,
    Ets,
>;

#[allow(dead_code)]
pub struct Tamagothci<'a> {
    display_device: DisplayDevice<'a>,
    display_spi: SpiDeviceDriver<'a, SpiDriver<'a>>,
    vib_motor_pin: PinDriver<'a, Gpio13, Output>,
}

impl<'a> Tamagothci<'a> {
    pub fn new() -> Self {
        let peripherals = Peripherals::take().unwrap();

        let mosi = peripherals.pins.gpio23;
        let sclk = peripherals.pins.gpio18;

        let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());
        let mut spi = SpiDeviceDriver::new_single(
            peripherals.spi2,
            sclk,
            mosi,
            Option::<gpio::Gpio12>::None,
            Dma::Disabled,
            Some(peripherals.pins.gpio5),
            &config,
        )
        .unwrap();

        let mut delay = delay::Ets;

        let rst = PinDriver::output(peripherals.pins.gpio9).unwrap();
        let dc = PinDriver::output(peripherals.pins.gpio10).unwrap();
        let busy = PinDriver::input(peripherals.pins.gpio19).unwrap();

        let device = Epd1in54::new(&mut spi, busy, dc, rst, &mut delay).unwrap();

        Tamagothci {
            display_device: device,
            display_spi: spi,
            vib_motor_pin: PinDriver::output(peripherals.pins.gpio13).unwrap(),
        }
    }

    pub fn draw(&mut self) {
        let mut faces = faces::Faces::new();

        let raw_image = faces.random().unwrap();
        let image = Image::new(&raw_image, Point::new(0, 60));
        let mut display = Display1in54::default();

        image.draw(&mut display).unwrap();

        self.display_device
            .set_lut(&mut self.display_spi, Some(RefreshLut::Quick))
            .unwrap();
        self.display_device
            .update_and_display_frame(&mut self.display_spi, display.buffer(), &mut Ets)
            .unwrap();
    }

    #[allow(dead_code)]
    pub fn vibrate_short(&mut self) {
        self.vib_motor_pin.set_high().unwrap();
        thread::sleep(time::Duration::from_millis(50));
        self.vib_motor_pin.set_low().unwrap();
    }
}
