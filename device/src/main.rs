#![no_std]
#![no_main]

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ufmt::{derive::uDebug, uWrite};

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

        // For some reason this serialization code works ...
        //
        // let message = "hElLo";
        // let bytes = [0x01, 0x10, 0x02, 0x20];
        // let output: postcard::Result<Vec<u8, 11>> = to_vec(&RefStruct {
        //     bytes: &bytes,
        //     str_s: message,
        // });

        // ... but this doesn't
        //
        // NOTE: Not sure how much memory this takes but 64 bytes should be
        // plenty for serialization, i guess
        // let output: postcard::Result<Vec<u8, 64>> = to_vec(&entry);

        use core::fmt::Write;

        // struct BufWriter {
        //     buf: [u8; 64],
        // }
        // impl Write for BufWriter {
        //     fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
        //         for (i, b) in s.as_bytes().iter().enumerate() {
        //             self.buf[i] = *b;
        //         }
        //         Ok(())
        //     }
        // }

        // let mut buf_writer = BufWriter { buf: [0; 64] };

        struct SerialWriter<'a, T> {
            serial: &'a mut T,
        }
        impl<'a, T> Write for SerialWriter<'a, T> where T: uWrite {
            fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> {
                ufmt::uwrite!(&mut self.serial, "{}", s).map_err(|_| core::fmt::Error::default())?;
                // for (i, b) in s.as_bytes().iter().enumerate() {
                //     self.buf[i] = *b;
                // }
                Ok(())
            }
        }

        let mut serial_writer = SerialWriter { serial: &mut serial };

        write!(&mut serial_writer, "{}", entry).unwrap();
        ufmt::uwriteln!(&mut serial, "").unwrap();

        ufmt::uwriteln!(&mut serial, "Writing").unwrap();

        // arduino_hal::delay_ms(1000);
        // for byte in buf_writer.buf {
        //     ufmt::uwriteln!(&mut serial, "{}", byte).unwrap();
        // }
        // 062144
        arduino_hal::delay_ms(5000);
    }
}
