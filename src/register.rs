//! Register address. see Table 7
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
