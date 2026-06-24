use crate::records::find_function_type_in::FindFunctionTypeIn;
use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;

impl FindFunctionTypeIn {
    pub fn visit_type_id_intersection_type(
        &mut self,
        _ty: TypeId,
        _itv: &IntersectionType,
    ) -> bool {
        true
    }
}
