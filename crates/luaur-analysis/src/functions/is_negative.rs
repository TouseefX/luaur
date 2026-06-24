use crate::enums::polarity::Polarity;

pub fn is_negative(p: Polarity) -> bool {
    (p as u8 & Polarity::Negative as u8) != 0
}
