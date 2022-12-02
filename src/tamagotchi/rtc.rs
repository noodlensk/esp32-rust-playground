use esp_idf_hal::gpio::{Gpio21, Gpio22};
use esp_idf_hal::i2c;
use esp_idf_hal::i2c::{I2cDriver, I2C0};
use esp_idf_hal::prelude::FromValueType;
use pcf8563::{DateTime, PCF8563};

pub struct Rtc<'a> {
    rtc: PCF8563<I2cDriver<'a>>,
}

impl<'a> Rtc<'a> {
    pub fn new(i2c0: I2C0, sda: Gpio21, scl: Gpio22) -> Result<Self, String> {
        let config = <i2c::config::Config as Default>::default().baudrate(400.kHz().into());
        let i2c = match I2cDriver::new(i2c0, sda, scl, &config) {
            Ok(i2c) => i2c,
            Err(error) => return Err(error.to_string()),
        };

        let rtc = PCF8563::new(i2c);

        Ok(Self { rtc })
    }

    pub fn current_time(&mut self) -> Result<DateTime, String> {
        match self.rtc.get_datetime() {
            Ok(time) => Ok(time),
            Err(_error) => Err(String::from("got error")),
        }
    }
}
