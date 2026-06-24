use crate::records::iterator::Iterator;
use crate::type_aliases::value_type_constraint_graph::ValueType;

impl Iterator {
    #[inline]
    pub fn operator_deref(&self) -> ValueType {
        let cl = unsafe { self.cl.as_ref() };
        cl.order[self.index].clone()
    }
}
