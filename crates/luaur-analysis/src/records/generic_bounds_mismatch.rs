use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericBoundsMismatch {
    pub(crate) generic_name: alloc::string::String,
    pub(crate) lower_bounds: alloc::vec::Vec<TypeId>,
    pub(crate) upper_bounds: alloc::vec::Vec<TypeId>,
}
