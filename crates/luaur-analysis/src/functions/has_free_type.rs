use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::has_free_type::HasFreeType;
use crate::type_aliases::type_id::TypeId;

pub fn has_free_type(ty: TypeId) -> bool {
    let mut hft = HasFreeType::new();
    hft.traverse_type_id(ty);
    hft.result
}
