use crate::records::negation_type::NegationType;
use crate::records::negation_type_finder::NegationTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl NegationTypeFinder {
    pub fn negation_type_finder_visit_type_id(&mut self, _ty: TypeId) -> bool {
        !self.found
    }

    pub fn negation_type_finder_visit_type_id_negation_type(
        &mut self,
        _ty: TypeId,
        _ntv: &NegationType,
    ) -> bool {
        self.found = true;
        !self.found
    }
}
