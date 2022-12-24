//! Types used in this crate

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
