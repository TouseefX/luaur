use crate::records::find_function_type_in::FindFunctionTypeIn;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl FindFunctionTypeIn {
    pub fn visit_type_id_union_type(&mut self, _ty: TypeId, _utv: &UnionType) -> bool {
        true
    }
}
