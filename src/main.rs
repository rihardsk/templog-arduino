#![no_std]
#![no_main]

use dht11::Dht11;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut delay = arduino_hal::Delay::new();
    let temp_pin = pins.d2.into_opendrain_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

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

    let mut dht11 = Dht11::new(temp_pin);

    arduino_hal::delay_ms(1000);
    loop {
        led.toggle();
        let measurement = dht11.perform_measurement(&mut delay);
        led.toggle();

        match measurement {
            Ok(m) => {
                ufmt::uwriteln!(&mut serial, "{}°, {}% RH", m.temperature, m.humidity).unwrap()
            }
            Err(dht11::Error::Timeout) => ufmt::uwriteln!(&mut serial, "Error: Timeout").unwrap(),
            Err(dht11::Error::CrcMismatch) => {
                ufmt::uwriteln!(&mut serial, "Error: Checksum mismatch").unwrap()
            }
            Err(dht11::Error::Gpio(_e)) => {
                ufmt::uwriteln!(&mut serial, "Error: Gpio error").unwrap()
            }
        };
        arduino_hal::delay_ms(5000);
    }
}
