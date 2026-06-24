#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Polarity {
    None = 0b000,
    Positive = 0b001,
    Negative = 0b010,
    Mixed = 0b011,
    Unknown = 0b100,
}

impl Polarity {
    pub const None: Self = Self::None;
    pub const Positive: Self = Self::Positive;
    pub const Negative: Self = Self::Negative;
    pub const Mixed: Self = Self::Mixed;
    pub const Unknown: Self = Self::Unknown;
}

impl Default for Polarity {
    fn default() -> Self {
        Self::None
    }
}
