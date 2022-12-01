use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;
use embedded_graphics::Drawable;

use epd_waveshare::epd1in54_v2::{Display1in54, Epd1in54};
use epd_waveshare::graphics::Display;
use epd_waveshare::prelude::{RefreshLut, WaveshareDisplay};

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Gpio10, Gpio18, Gpio19, Gpio23, Gpio5, Gpio9, Input, Output, PinDriver};
use esp_idf_hal::spi::{Dma, SpiDeviceDriver, SpiDriver, SPI2};
use esp_idf_hal::prelude::FromValueType;
use esp_idf_hal::{delay, gpio, spi};

use crate::tamagotchi::faces;

type Device<'a> = Epd1in54<
    SpiDeviceDriver<'a, SpiDriver<'a>>,
    PinDriver<'a, Gpio19, Input>,
    PinDriver<'a, Gpio10, Output>,
    PinDriver<'a, Gpio9, Output>,
    Ets,
>;

pub struct EInk<'a> {
    device: Device<'a>,
    spi: SpiDeviceDriver<'a, SpiDriver<'a>>,
}

impl<'a> EInk<'a> {
    pub fn new(
        spi2: SPI2,
        mosi: Gpio23,
        sclk: Gpio18,
        cs: Gpio5,
        rst: Gpio9,
        dc: Gpio10,
        busy: Gpio19,
    ) -> Self {
        let config = <spi::config::Config as Default>::default().baudrate(26.MHz().into());
        let mut spi = SpiDeviceDriver::new_single(
            spi2,
            sclk,
            mosi,
            Option::<gpio::Gpio12>::None,
            Dma::Disabled,
            Some(cs),
            &config,
        )
            .unwrap();

        let mut delay = delay::Ets;

        let rst = PinDriver::output(rst).unwrap();
        let dc = PinDriver::output(dc).unwrap();
        let busy = PinDriver::input(busy).unwrap();

        let device = Epd1in54::new(&mut spi, busy, dc, rst, &mut delay).unwrap();

        EInk { device, spi }
    }

    pub fn draw_random_face(&mut self) -> Result<(), String> {
        let mut faces = faces::Faces::new();

        let raw_image = match faces.random() {
            Ok(img) => img,
            Err(e) => return Err(e),
        };

        let image = Image::new(&raw_image, Point::new(0, 60));
        let mut display = Display1in54::default();

        if let Err(error) = image.draw(&mut display) {
            return Err(error.to_string());
        }

        if let Err(error) = self.device.set_lut(&mut self.spi, Some(RefreshLut::Quick)) {
            return Err(error.to_string());
        };

        if let Err(error) =
            self.device
                .update_and_display_frame(&mut self.spi, display.buffer(), &mut Ets)
        {
            return Err(error.to_string());
        };

        Ok(())
    }
}
