use crate::enums::polarity::Polarity;

#[allow(non_snake_case)]
pub fn operator_bitand(lhs: Polarity, rhs: Polarity) -> Polarity {
    unsafe { core::mem::transmute(lhs as u8 & rhs as u8) }
}

impl core::ops::BitAnd for Polarity {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        operator_bitand(self, rhs)
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitand as operator_bitand_polarity_polarity;
