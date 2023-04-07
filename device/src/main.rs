#![no_std]
#![no_main]

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use heapless::Vec;
use panic_halt as _;
use postcard::to_vec;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 9600);

    ufmt::uwriteln!(&mut serial, "Serial set up, entering loop").unwrap();

    arduino_hal::delay_ms(1000);
    loop {
        let entry = NaiveDateTime::MIN;

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
        // uwriteln! calls (ok, that was the case previously, now it simply
        // resets and doesn't get to the next uwriteln)
        //
        // Not sure how much memory this takes but 64 bytes should be plenty for
        // serialization, i guess
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

        arduino_hal::delay_ms(5000);
    }
}
