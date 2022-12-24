use crate::register::Register;
use crate::types::{BlinkMode, BlinkRate, ConfigBitFlag, DecodeMode, DigitType, PinMode};
use crate::Max6955;
use crate::DEFAULT_SLAVE_ADDR;
use bit_field::BitField;
use embedded_hal_async::i2c::I2c;

impl<I2C, E> Max6955<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Construct a new MAX6955 driver instance with I2C peripheral and default address of `0x60`.
    ///
    /// # Arguments
    ///
    /// * `i2c` - I2C interface
    ///
    pub fn new(i2c: I2C) -> Self {
        Max6955 {
            i2c,
            addr: DEFAULT_SLAVE_ADDR,
        }
    }

    /// Construct a new MAX6955 driver instance with I2C peripheral and address.
    ///
    /// # Arguments
    ///
    /// * `i2c` - I2C interface
    /// * `addr` - device address. This can be `0x60` ~ `0x6F`. See table 5 in the datasheet.
    ///
    pub fn with_address(i2c: I2C, addr: u8) -> Self {
        Max6955 { i2c, addr }
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
    pub async fn set_global_intensity(&mut self, intensity: u8) -> Result<(), E> {
        self.write_register(Register::GlobalIntensity, intensity)
            .await
    }

    /// Control Blinking
    /// # Arguments
    ///
    /// * `mode` - `BlinkMode::Enable`: blink, `BlinkMode::Disable`: not blink
    /// * `rate` - `BlinkRate::Fast`: 0.5s cycle, `BlinkRate::Slow`: 1.0s cycle
    pub async fn set_blink(&mut self, mode: BlinkMode, rate: BlinkRate) -> Result<(), E> {
        self.set_configuration_bit(ConfigBitFlag::Blink, mode.value())
            .await?;
        self.set_configuration_bit(ConfigBitFlag::BlinkRate, rate.value())
            .await
    }

    /// Power up Display
    pub async fn powerup(&mut self) -> Result<(), E> {
        self.set_configuration_bit(ConfigBitFlag::Shutdown, true)
            .await
    }

    /// Shutdown Display
    pub async fn shutdown(&mut self) -> Result<(), E> {
        self.set_configuration_bit(ConfigBitFlag::Shutdown, false)
            .await
    }

    /// Configure Digit Type
    /// # Arguments
    ///
    /// * `digit_type` - one of four `DigitType`s
    pub async fn set_digit_type(&mut self, digit_type: DigitType) -> Result<(), E> {
        self.write_register(Register::DigitType, digit_type.value())
            .await
    }

    /// Configure Pin Mode
    /// # Arguments
    ///
    /// * `port` - `0` ~ `4`
    /// * `pin_mode`
    pub async fn set_pin_mode(&mut self, port: usize, pin_mode: PinMode) -> Result<(), E> {
        let mut port_config: u8 = self.read_register(Register::PortConfiguration).await?;
        let config = match pin_mode {
            PinMode::Input => *port_config.set_bit(port, true),
            PinMode::Output => *port_config.set_bit(port, false),
        };
        self.write_register(Register::PortConfiguration, config)
            .await
    }

    /// Configure Decode Mode
    /// # Arguments
    /// * `mode` - `DecodeMode`
    pub async fn set_decode_mode(&mut self, mode: DecodeMode) -> Result<(), E> {
        self.write_register(Register::DecodeMode, mode.value())
            .await
    }

    /// Display Test function
    /// # Arguments
    /// * `enable` - `true`: enable test
    pub async fn test(&mut self, enable: bool) -> Result<(), E> {
        if enable {
            self.write_register(Register::DisplayTest, 0x01).await
        } else {
            self.write_register(Register::DisplayTest, 0x00).await
        }
    }

    /// Clear Display
    pub async fn clear_display(&mut self) -> Result<(), E> {
        self.write_str("").await
    }

    /// Write Text
    /// # Arguments
    /// * `text` - text to write
    pub async fn write_str(&mut self, text: &str) -> Result<(), E> {
        let mut row: [u8; 9] = [b' '; 9];
        row[0] = Register::Digit0Plane0.addr();
        for (i, c) in text.chars().enumerate() {
            row[i + 1] = match c {
                ' '..='~' => c as u8,
                _ => b' ',
            }
        }
        self.i2c.write(self.addr, &row).await
    }

    async fn write_register(&mut self, reg: Register, byte: u8) -> Result<(), E> {
        self.i2c.write(self.addr, &[reg.addr(), byte]).await
    }

    async fn set_configuration_bit(&mut self, bit: ConfigBitFlag, set: bool) -> Result<(), E> {
        let mut config: u8 = self.read_register(Register::Configuration).await?;
        config.set_bit(bit.value(), set);
        self.write_register(Register::Configuration, config).await
    }

    async fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buffer: [u8; 8] = [0; 8];
        self.read_registers(reg, &mut buffer).await?;
        Ok(buffer[0])
    }

    async fn read_registers(&mut self, reg: Register, buffer: &mut [u8; 8]) -> Result<(), E> {
        self.i2c.write_read(self.addr, &[reg.addr()], buffer).await
    }
}
