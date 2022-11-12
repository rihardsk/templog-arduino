#![no_std]
#![no_main]

use dht_sensor::{dht11, DhtReading};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut delay = arduino_hal::Delay::new();
    let mut temp_pin = pins.d2.into_opendrain_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    loop {
        match dht11::Reading::read(&mut delay, &mut temp_pin) {
            Ok(dht11::Reading {
                temperature,
                relative_humidity,
            }) => {
                ufmt::uwriteln!(&mut serial, "{}°, {}% RH", temperature, relative_humidity).unwrap()
            }
            Err(e) => match e {
                dht_sensor::DhtError::PinError(_) => ufmt::uwriteln!(&mut serial, "Error: Pin error").unwrap(),
                dht_sensor::DhtError::ChecksumMismatch => ufmt::uwriteln!(&mut serial, "Error: Checksum mismatch").unwrap(),
                dht_sensor::DhtError::Timeout => ufmt::uwriteln!(&mut serial, "Error: Timeout").unwrap(),
            },
            // ufmt::uwriteln!(&mut serial, "Error {}", "aa").unwrap(),
        }
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}
