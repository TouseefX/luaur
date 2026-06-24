use crate::enums::polarity::Polarity;

#[allow(non_snake_case)]
pub fn operator_bitor_assign(lhs: &mut Polarity, rhs: Polarity) -> &mut Polarity {
    *lhs = crate::functions::operator_bitor_polarity::operator_bitor(*lhs, rhs);
    lhs
}

impl core::ops::BitOrAssign for Polarity {
    fn bitor_assign(&mut self, rhs: Self) {
        operator_bitor_assign(self, rhs);
    }
}
