use crate::records::symbol::Symbol;

impl Symbol {
    #[inline]
    pub fn operator_lt_symbol(&self, rhs: &Symbol) -> bool {
        if !self.local.is_null() && !rhs.local.is_null() {
            return (self.local as usize) < (rhs.local as usize);
        } else if !self.global.value.is_null() && !rhs.global.value.is_null() {
            return self.global.operator_lt(&rhs.global);
        } else if !self.local.is_null() {
            return true;
        }

        false
    }
}
