#![no_std]

use chrono::{Datelike, Timelike, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ufmt::derive::uDebug;

#[derive(Deserialize)]
pub enum Command {
    ReadTempsSince(NaiveDateTime),
    SetCurrentTime(NaiveDateTime),
}

#[derive(Serialize, Copy, Clone, uDebug)]
pub struct TempReading {
    pub temperature: i16,
    pub relative_humidity: u16,
}

#[cfg(feature = "dht11")]
impl From<dht11::Measurement> for TempReading {
    fn from(m: dht11::Measurement) -> Self {
        TempReading {
            temperature: m.temperature,
            relative_humidity: m.humidity,
        }
    }
}

#[derive(Serialize, Copy, Clone, uDebug)]
pub enum TempError {
    Timeout,
    CrcMismatch,
    Gpio,
}

// I've copied these from the ds323x lib but don't know what all of them mean
// exactly
#[derive(Serialize, Copy, Clone, uDebug)]
pub enum TimeError {
    Comm,
    Pin,
    InvalidInputData,
    InvalidDeviceState,
}

#[cfg(feature = "ds323x")]
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

#[cfg(feature = "dht11")]
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
pub struct FNaiveDateTime(pub NaiveDateTime);

impl ufmt::uDebug for FNaiveDateTime {
    fn fmt<W>(&self, f: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized,
    {
        let d = self.0.date();
        ufmt::uDebug::fmt(&d.year(), f)?;
        f.write_char('.')?;
        ufmt::uDebug::fmt(&d.month(), f)?;
        f.write_char('.')?;
        ufmt::uDebug::fmt(&d.day(), f)?;

        f.write_char('_')?;

        let t = self.0.time();
        ufmt::uDebug::fmt(&t.hour(), f)?;
        f.write_char(':')?;
        ufmt::uDebug::fmt(&t.minute(), f)?;
        f.write_char(':')?;
        ufmt::uDebug::fmt(&t.second(), f)?;
        Ok(())
    }
}

#[derive(Serialize, Copy, Clone, uDebug)]
pub struct TempEntry {
    pub reading: Result<TempReading, TempError>,
    pub time: Result<FNaiveDateTime, TimeError>,
}
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
