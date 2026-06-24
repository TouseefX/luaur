use crate::records::function_type::FunctionType;
use crate::records::generic_type_finder::GenericTypeFinder;
use crate::type_aliases::type_id::TypeId;

impl GenericTypeFinder {
    pub fn visit_type_id_luau_function_type(&mut self, _ty: TypeId, ftv: &FunctionType) -> bool {
        if ftv.has_no_free_or_generic_types {
            return false;
        }
        if !ftv.generics.is_empty() || !ftv.generic_packs.is_empty() {
            self.found = true;
        }
        !self.found
    }
}
