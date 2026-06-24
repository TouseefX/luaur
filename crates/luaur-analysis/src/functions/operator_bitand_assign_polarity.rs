use crate::enums::polarity::Polarity;

#[allow(non_snake_case)]
pub fn operator_bitand_assign(lhs: &mut Polarity, rhs: Polarity) -> &mut Polarity {
    *lhs = crate::functions::operator_bitand_polarity::operator_bitand(*lhs, rhs);
    lhs
}

impl core::ops::BitAndAssign for Polarity {
    fn bitand_assign(&mut self, rhs: Self) {
        operator_bitand_assign(self, rhs);
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use operator_bitand_assign as operator_bitand_assign_polarity_polarity;
