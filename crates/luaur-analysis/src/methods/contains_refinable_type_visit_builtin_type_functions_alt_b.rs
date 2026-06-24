use crate::records::contains_refinable_type::ContainsRefinableType;
use crate::records::no_refine_type::NoRefineType;
use crate::type_aliases::type_id::TypeId;

impl ContainsRefinableType {
    pub fn visit_type_id_no_refine_type(&mut self, _ty: TypeId, _nrt: &NoRefineType) -> bool {
        false
    }
}
