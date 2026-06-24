use crate::enums::polarity::Polarity;

#[allow(non_snake_case)]
pub fn operator_bitor(lhs: Polarity, rhs: Polarity) -> Polarity {
    unsafe { core::mem::transmute(lhs as u8 | rhs as u8) }
}

impl core::ops::BitOr for Polarity {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        operator_bitor(self, rhs)
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitor as operator_bitor_polarity_polarity;
