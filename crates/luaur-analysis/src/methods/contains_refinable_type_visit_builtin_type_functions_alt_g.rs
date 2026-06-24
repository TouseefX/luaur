use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_intersection_type(
        &mut self,
        _ty: TypeId,
        _intersection: &IntersectionType,
    ) -> bool {
        !self.found
    }
}
