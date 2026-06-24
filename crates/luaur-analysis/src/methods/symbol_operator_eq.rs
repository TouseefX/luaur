use crate::records::symbol::Symbol;

impl Symbol {
    #[inline]
    pub fn operator_eq_symbol(&self, rhs: &Self) -> bool {
        if !self.local.is_null() {
            return self.local == rhs.local;
        } else if !self.global.value.is_null() {
            return !rhs.global.value.is_null() && self.global.operator_eq_c_char(rhs.global.value);
        } else {
            return rhs.local.is_null() && rhs.global.value.is_null();
        }
    }
}
