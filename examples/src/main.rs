#![no_std]
#![no_main]

use embedded_dht_rs::{dht11::Dht11};
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl, delay::Delay, gpio::{Io, Level, OutputOpenDrain, Pull}, i2c::I2C, peripherals::Peripherals, prelude::*, system::SystemControl
};
use fugit::HertzU32;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    esp_println::logger::init_logger_from_env();

    let delay = Delay::new(&clocks);

    let od_for_dht11 = OutputOpenDrain::new(io.pins.gpio4, Level::High, Pull::None);
    let mut dht11 = Dht11::new(od_for_dht11, delay);

    loop {
        delay.delay(5000.millis());
        match dht11.read() {
            Ok(sensor_reading) => log::info!(
                "DHT 11 Sensor - Temperature: {} °C, humidity: {} %",
                sensor_reading.temperature,
                sensor_reading.humidity
            ),
            Err(error) => log::error!("An error occurred while trying to read sensor: {:?}", error),
        }

        log::info!("-----");
    }
}