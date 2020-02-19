//! A platform agnostic driver to interface with MAX6955 LED Display Driver
//!
//! This driver was built using [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal/~0.2
//!
//! ### Datasheets
//! - [MAX6955](https://datasheets.maximintegrated.com/en/ds/MAX6955.pdf)
//!
//! # Examples
//!```rust
//!#[entry]
//!fn main() -> ! {
//!    let dp = stm32f30x::Peripherals::take().unwrap();
//!    let mut flash = dp.FLASH.constrain();
//!    let mut rcc = dp.RCC.constrain();
//!
//!    let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!
//!    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
//!    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//!    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//!
//!    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
//!
//!    // create an instance with the default address 0x60
//!    let mut max6955 = Max6955::new(i2c).unwrap();
//!    // power up
//!    max6955.powerup().unwrap();
//!    // set intensity
//!    max6955.set_global_intensity(4).unwrap();
//!    // write text
//!    max6955.write_str("HELLO").unwrap();
//!
//!    loop {}
//!}
//!```

// #![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate bit_field;
extern crate embedded_hal as hal;

use bit_field::BitField;
use hal::blocking::i2c::{Write, WriteRead};

/// Default address
pub const DEFAULT_SLAVE_ADDR: u8 = 0x60;

/// MAX6955 driver
pub struct Max6955<I2C> {
    i2c: I2C,
    addr: u8,
}

impl<I2C, E> Max6955<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
{
    /// Construct a new MAX6955 driver instance with I2C peripheral and default address of `0x60`.
    ///
    /// # Arguments
    ///
    /// * `i2c` - I2C interface
    ///
    /// # Errors
    ///
    /// * `E` - returned in case there was an error reading/writing to the device
    ///
    pub fn new(i2c: I2C) -> Result<Self, E> {
        let max6955 = Max6955 {
            i2c,
            addr: DEFAULT_SLAVE_ADDR,
        };
        Ok(max6955)
    }

    /// Construct a new MAX6955 driver instance with I2C peripheral and address.
    ///
    /// # Arguments
    ///
    /// * `i2c` - I2C interface
    /// * `addr` - device address. This can be `0x60` ~ `0x6F`. See table 5 in the datasheet.
    ///
    /// # Errors
    ///
    /// * `E` - returned in case there was an error reading/writing to the device
    ///
    pub fn with_address(i2c: I2C, addr: u8) -> Result<Self, E> {
        let max6955 = Max6955 { i2c, addr };
        Ok(max6955)
    }

    /// Set device address
    /// # Arguments
    ///
    /// * `addr` - device address. This can be `0x60` ~ `0x6F`. See table 5 in the datasheet.
    pub fn set_address(&mut self, addr: u8) {
        self.addr = addr;
    }

    /// Set Global Intensity
    /// # Arguments
    ///
    /// * `intensity` - intensity level `0`: lowest ~ `15`: highest
    pub fn set_global_intensity(&mut self, intensity: u8) -> Result<(), E> {
        self.write_register(Register::GlobalIntensity, intensity)?;
        Ok(())
    }

    /// Control Blinking
    /// # Arguments
    ///
    /// * `mode` - `BlinkMode::Enable`: blink, `BlinkMode::Disable`: not blink
    /// * `rate` - `BlinkRate::Fast`: 0.5s cycle, `BlinkRate::Slow`: 1.0s cycle
    pub fn set_blink(&mut self, mode: BlinkMode, rate: BlinkRate) -> Result<(), E> {
        self.set_configuration_bit(ConfigBitFlag::Blink, mode.value())?;
        self.set_configuration_bit(ConfigBitFlag::BlinkRate, rate.value())
    }

    /// Power up Display
    pub fn powerup(&mut self) -> Result<(), E> {
        self.set_configuration_bit(ConfigBitFlag::Shutdown, true)
    }

    /// Shutdown Display
    pub fn shutdown(&mut self) -> Result<(), E> {
        self.set_configuration_bit(ConfigBitFlag::Shutdown, false)
    }

    /// Configure Digit Type
    /// # Arguments
    ///
    /// * `digit_type` - one of four `DigitType`s
    pub fn set_digit_type(&mut self, digit_type: DigitType) -> Result<(), E> {
        self.write_register(Register::DigitType, digit_type.value())
    }

    /// Configure Pin Mode
    /// # Arguments
    ///
    /// * `port` - `0` ~ `4`
    /// * `pin_mode`
    pub fn set_pin_mode(&mut self, port: usize, pin_mode: PinMode) -> Result<(), E> {
        let mut port_config: u8 = self.read_register(Register::PortConfiguration)?;
        let config = match pin_mode {
            PinMode::Input => *port_config.set_bit(port, true),
            PinMode::Output => *port_config.set_bit(port, false),
        };
        self.write_register(Register::PortConfiguration, config)
    }

    /// Configure Decode Mode
    /// # Arguments
    /// * `mode` - `DecodeMode`
    pub fn set_decode_mode(&mut self, mode: DecodeMode) -> Result<(), E> {
        self.write_register(Register::DecodeMode, mode.value())
    }

    /// Display Test function
    /// # Arguments
    /// * `enable` - `true`: enable test
    pub fn test(&mut self, enable: bool) -> Result<(), E> {
        if enable {
            self.write_register(Register::DisplayTest, 0x01)
        } else {
            self.write_register(Register::DisplayTest, 0x00)
        }
    }

    /// Clear Display
    pub fn clear_display(&mut self) -> Result<(), E> {
        self.write_str("")
    }

    /// Write Text
    /// # Arguments
    /// * `text` - text to write
    pub fn write_str(&mut self, text: &str) -> Result<(), E> {
        let mut row: [u8; 9] = [b' '; 9];
        row[0] = Register::Digit0Plane0.addr();
        for (i, c) in text.chars().enumerate() {
            row[i + 1] = match c {
                ' '..='~' => c as u8,
                _ => b' ',
            }
        }
        self.i2c.write(self.addr, &row)
    }

    fn write_register(&mut self, reg: Register, byte: u8) -> Result<(), E> {
        self.i2c.write(self.addr, &[reg.addr(), byte])
    }

    fn set_configuration_bit(&mut self, bit: ConfigBitFlag, set: bool) -> Result<(), E> {
        let mut config: u8 = self.read_register(Register::Configuration)?;
        config.set_bit(bit.value(), set);
        self.write_register(Register::Configuration, config)
    }

    fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_registers(reg, &mut buffer)?;
        Ok(buffer[0])
    }

    fn read_registers(&mut self, reg: Register, buffer: &mut [u8; 8]) -> Result<(), E> {
        self.i2c.write_read(self.addr, &[reg.addr()], buffer)
    }
}

/// Register address. see Table 7
pub enum Register {
    NoOp = 0x00,
    DecodeMode = 0x01,
    GlobalIntensity = 0x02,
    ScanLimit = 0x03,
    Configuration = 0x04,
    GpioData = 0x05,
    PortConfiguration = 0x06,
    DisplayTest = 0x07,
    KeyAMaskDebounce = 0x08,
    KeyBMaskDebounce = 0x09,
    KeyCMaskDebounce = 0x0A,
    KeyDMaskDebounce = 0x0B,
    DigitType = 0x0C,
    KeyBPressed = 0x0D,
    KeyCPressed = 0x0E,
    KeyDPressed = 0x0F,
    Intensity10 = 0x10,
    Intensity32 = 0x11,
    Intensity54 = 0x12,
    Intensity76 = 0x13,
    Intensity10a = 0x14,
    Intensity32a = 0x15,
    Intensity54a = 0x16,
    Intensity76a = 0x17,
    Digit0Plane0 = 0x20,
    Digit1Plane0 = 0x21,
    Digit2Plane0 = 0x22,
    Digit3Plane0 = 0x23,
    Digit4Plane0 = 0x24,
    Digit5Plane0 = 0x25,
    Digit6Plane0 = 0x26,
    Digit7Plane0 = 0x27,
    Digit0Plane1 = 0x40,
    Digit1Plane1 = 0x41,
    Digit2Plane1 = 0x42,
    Digit3Plane1 = 0x43,
    Digit4Plane1 = 0x44,
    Digit5Plane1 = 0x45,
    Digit6Plane1 = 0x46,
    Digit7Plane1 = 0x47,
}

impl Register {
    /// return register address as u8
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// Configuration Register bits. see Table 17
pub enum ConfigBitFlag {
    Shutdown = 0x00,
    BlinkRate = 0x02,
    Blink = 0x03,
    BlinkTiming = 0x04,
    ClearDigit = 0x05,
    Intensity = 0x06,
    BlinkPhase = 0x07,
}

impl ConfigBitFlag {
    /// return enum value as usize
    pub fn value(self) -> usize {
        self as usize
    }
}

/// Display Digit Configuration. see Table 14
pub enum DigitType {
    /// Digits 7 to 0 are 16-segment or 7- segment digits.
    Seg7_16 = 0x00,
    /// Digit 0 is a 14-segment digit, digits 7 to 1 are 16-segment or 7- segment digits.
    D0_14 = 0x01,
    /// Digits 2 to 0 are 14-segment digits, digits 7 to 3 are 16- segment or 7-segment digits.
    D0D2_14 = 0x07,
    /// Digits 7 to 0 are 14-segment digits.
    Seg14 = 0xFF,
}

impl DigitType {
    /// return enum value as u8
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Decode Mode. see Table 15
pub enum DecodeMode {
    /// No decode for digit pairs 7 to 0.
    NoDecode = 0x00,
    /// Hexadecimal decode for digit pair 0, no decode for digit pairs 7 to 1.
    HexD0 = 0x01,
    /// Hexadecimal decode for digit pairs 2 to 0, no decode for digit pairs 7 to 3.
    HexD0D2 = 0x07,
    /// Hexadecimal decode for digit pairs 7 to 0.
    Hex = 0xFF,
}

impl DecodeMode {
    /// return enum value as u8
    pub fn value(self) -> u8 {
        self as u8
    }
}

/// Pin Mode Input/Output
pub enum PinMode {
    Input,
    Output,
}

/// Blink Mode Enable/Disable
pub enum BlinkMode {
    Disable,
    Enable,
}

impl BlinkMode {
    /// return enum value as bool
    pub fn value(self) -> bool {
        match self {
            BlinkMode::Disable => false,
            BlinkMode::Enable => true,
        }
    }
}

/// Blink Rate Fast/Slow
pub enum BlinkRate {
    Fast,
    Slow,
}

impl BlinkRate {
    /// return enum value as bool
    pub fn value(self) -> bool {
        match self {
            BlinkRate::Slow => false,
            BlinkRate::Fast => true,
        }
    }
}
