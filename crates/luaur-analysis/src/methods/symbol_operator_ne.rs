use crate::records::symbol::Symbol;

impl Symbol {
    #[inline]
    pub fn operator_ne_symbol(&self, rhs: &Self) -> bool {
        !self.operator_eq_symbol(rhs)
    }
}
