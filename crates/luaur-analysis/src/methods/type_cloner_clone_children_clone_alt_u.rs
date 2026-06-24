use crate::records::type_cloner::TypeCloner;
use crate::records::unknown_type::UnknownType;

impl TypeCloner {
    pub fn clone_children_unknown_type(&mut self, _t: *mut UnknownType) {
        // noop.
    }
}
