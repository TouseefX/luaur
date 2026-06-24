use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _union: &UnionType) -> bool {
        !self.found
    }
}
