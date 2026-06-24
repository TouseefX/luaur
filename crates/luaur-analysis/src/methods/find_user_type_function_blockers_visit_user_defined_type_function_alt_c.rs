use crate::records::extern_type::ExternType;
use crate::records::find_user_type_function_blockers::FindUserTypeFunctionBlockers;
use crate::type_aliases::type_id::TypeId;

impl FindUserTypeFunctionBlockers {
    pub fn visit_type_id_extern_type(&mut self, _ty: TypeId, _extern: &ExternType) -> bool {
        false
    }
}
