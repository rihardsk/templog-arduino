// #![no_std]

use std::{io::{self, Write}, thread, time::Duration};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use heapless::Vec;
use postcard::to_vec;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct RefStruct<'a> {
    bytes: &'a [u8],
    str_s: &'a str,
}

fn main() -> ! {
    let mut stdout = io::stdout().lock();
    println!("Serial set up, entering loop");

    loop {
        let entry = NaiveDateTime::MIN;

        println!("Converting");
        thread::sleep(Duration::from_secs(1));

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
        // The above is true on AVR, on x86 this code works as expected
        //
        // Not sure how much memory this takes but 64 bytes should be plenty for
        // serialization, i guess
        let output: postcard::Result<Vec<u8, 64>> = to_vec(&entry);

        println!("Writing");
        thread::sleep(Duration::from_secs(1));
        match output {
            Ok(output) => {
                for byte in output {
                    stdout.write_all(&[byte]).unwrap();
                }
                stdout.flush().unwrap();
            }
            Err(_e) => println!("Serialization error"),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
