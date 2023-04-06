#![no_std]
#![no_main]

use chrono::NaiveDateTime;
use dht11::Dht11;
use ds323x::DateTimeAccess;
use heapless::Vec;
use panic_halt as _;
use postcard::to_vec;
use serde::{Deserialize, Serialize};

use templog_common::{FNaiveDateTime, TempEntry, TempError, TempReading, TimeError};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut delay = arduino_hal::Delay::new();
    let temp_pin = pins.d2.into_opendrain_high();

    // let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    ufmt::uwriteln!(&mut serial, "Setting up sensors").unwrap();

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

    let i2c = arduino_hal::I2c::new(
        dp.TWI, // don't know what this thing is
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    let mut rtc = ds323x::Ds323x::new_ds3231(i2c);

    ufmt::uwriteln!(&mut serial, "Entering loop").unwrap();

    arduino_hal::delay_ms(1000);
    loop {
        led.toggle();
        // let reading: Result<TempReading, TempError> = dht11
        //     .perform_measurement(&mut delay)
        //     .map(Into::into)
        //     .map_err(Into::into);
        // let time: Result<_, TimeError> = rtc.datetime().map(FNaiveDateTime).map_err(Into::into);
        led.toggle();

        // let entry = TempEntry { reading, time };
        let entry = TempEntry {
            reading: Ok(TempReading {
                temperature: 1,
                relative_humidity: 2,
            }),
            time: Ok(FNaiveDateTime(NaiveDateTime::MIN)),
        };

        ufmt::uwriteln!(&mut serial, "Converting").unwrap();
        arduino_hal::delay_ms(1000);

        // TODO: wtf, for some reason this serialization code works ...
        // let message = "hElLo";
        // let bytes = [0x01, 0x10, 0x02, 0x20];
        // let output: postcard::Result<Vec<u8, 11>> = to_vec(&RefStruct {
        //     bytes: &bytes,
        //     str_s: message,
        // });

        // TODO: ... but if we use this code, then it messes up even other
        // uwriteln! calls
        //
        // TempEntry suposedly takes 24 bytes in memory, 64 bytes should be
        // plenty for serialization
        let output: postcard::Result<Vec<u8, 64>> = to_vec(&entry);

        ufmt::uwriteln!(&mut serial, "Writing").unwrap();
        arduino_hal::delay_ms(1000);
        match output {
            Ok(output) => {
                for byte in output {
                    serial.write_byte(byte);
                }
                serial.flush();
            }
            Err(_e) => ufmt::uwriteln!(&mut serial, "Serialization error").unwrap(),
        }

        arduino_hal::delay_ms(1000);

        // ufmt::uwriteln!(&mut serial, "{:?}", entry.reading).unwrap();
        // match entry.time {
        //     Ok(t) => ufmt::uwriteln!(
        //         &mut serial,
        //         "{}:{}:{}",
        //         t.0.time().hour(),
        //         t.0.time().minute(),
        //         t.0.time().second()
        //     )
        //     .unwrap(),
        //     Err(_e) => ufmt::uwriteln!(&mut serial, "Time error").unwrap(),
        // }
        // match measurement {
        //     Ok(m) => {
        //         ufmt::uwriteln!(&mut serial, "{}°, {}% RH", m.temperature, m.humidity).unwrap()
        //     } // Err(dht11::Error::Timeout) => ufmt::uwriteln!(&mut serial, "Error: Timeout").unwrap(),
        //       // Err(dht11::Error::CrcMismatch) => {
        //       //     ufmt::uwriteln!(&mut serial, "Error: Checksum mismatch").unwrap()
        //       // }
        //       // Err(dht11::Error::Gpio(_e)) => {
        //       //     ufmt::uwriteln!(&mut serial, "Error: Gpio error").unwrap()
        //       // }
        // };
        arduino_hal::delay_ms(5000);
    }
}
