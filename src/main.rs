#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read;
use chrono::Timelike;
use dht11::{Dht11, Measurement};
use ds323x::{DateTimeAccess, NaiveDateTime, Rtcc};
use nb::try_nb;
use panic_halt as _;
use serde::{Deserialize, Serialize};
use ufmt::{derive::uDebug, uDisplay};

#[derive(Deserialize)]
enum Command {
    ReadTempsSince(NaiveDateTime),
    SetCurrentTime(NaiveDateTime),
}

#[derive(Serialize, Copy, Clone, uDebug)]
struct TempReading {
    temperature: i16,
    relative_humidity: u16,
}

impl From<Measurement> for TempReading {
    fn from(m: Measurement) -> Self {
        TempReading {
            temperature: m.temperature,
            relative_humidity: m.humidity,
        }
    }
}

#[derive(Serialize, Copy, Clone, uDebug)]
enum TempError {
    Timeout,
    CrcMismatch,
    Gpio,
}

// I've copied these from the ds323x lib but don't know what all of them mean
// exactly
#[derive(Serialize, Copy, Clone, uDebug)]
enum TimeError {
    Comm,
    Pin,
    InvalidInputData,
    InvalidDeviceState,
}

impl<T1, T2> From<ds323x::Error<T1, T2>> for TimeError {
    fn from(value: ds323x::Error<T1, T2>) -> Self {
        match value {
            ds323x::Error::Comm(_) => TimeError::Comm,
            ds323x::Error::Pin(_) => TimeError::Pin,
            ds323x::Error::InvalidInputData => TimeError::InvalidInputData,
            ds323x::Error::InvalidDeviceState => TimeError::InvalidDeviceState,
        }
    }
}

impl<T> From<dht11::Error<T>> for TempError {
    fn from(value: dht11::Error<T>) -> Self {
        match value {
            dht11::Error::Timeout => TempError::Timeout,
            dht11::Error::CrcMismatch => TempError::CrcMismatch,
            dht11::Error::Gpio(_) => TempError::Gpio,
        }
    }
}

// #[derive(Serialize, Copy, Clone)]
// enum These<A, B> {
//     This(A),
//     That(B),
//     Both(A, B),
// }

#[derive(Serialize, Copy, Clone)]
struct TempEntry {
    reading: Result<TempReading, TempError>,
    time: Result<NaiveDateTime, TimeError>,
}

const N_READINGS: usize = 100;
static mut READINGS: [Option<TempEntry>; N_READINGS] = [None; N_READINGS];
static mut NEXT_WRITE_POS: usize = 0;

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

    let i2c = arduino_hal::I2c::new(
        dp.TWI, // don't know what this thing is
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    let mut rtc = ds323x::Ds323x::new_ds3231(i2c);

    let mut coms_buff: [u8; 20] = [0; 20];

    arduino_hal::delay_ms(1000);
    loop {
        led.toggle();
        let reading: Result<TempReading, TempError> = dht11
            .perform_measurement(&mut delay)
            .map(Into::into)
            .map_err(Into::into);
        let time: Result<_, TimeError> = rtc.datetime().map_err(Into::into);
        led.toggle();

        let entry = TempEntry { reading, time };
        // TODO: use Cells or something instead. This should be fine for now,
        // though, as long as we don't mess up with interrupts or something
        unsafe {
            READINGS[NEXT_WRITE_POS] = Some(entry);
            NEXT_WRITE_POS = (NEXT_WRITE_POS + 1) % N_READINGS;
        }


        ufmt::uwriteln!(&mut serial, "{:?}", entry.reading).unwrap();
        match entry.time {
            Ok(t) => ufmt::uwriteln!(&mut serial, "{}:{}:{}", t.time().hour(), t.time().minute(), t.time().second()).unwrap(),
            Err(e) => ufmt::uwriteln!(&mut serial, "Time error").unwrap(),
        }
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
