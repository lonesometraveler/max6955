#![doc = include_str!("../README.md")]
// #![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
mod implementations;
pub mod register;
pub mod types;

/// Default address
pub const DEFAULT_SLAVE_ADDR: u8 = 0x60;

/// MAX6955 driver
pub struct Max6955<I2C> {
    i2c: I2C,
    addr: u8,
}
