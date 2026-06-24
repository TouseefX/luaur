use crate::records::internal_type_finder::InternalTypeFinder;
use crate::records::type_function_instance_type_pack::TypeFunctionInstanceTypePack;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl InternalTypeFinder {
    pub fn visit_type_pack_id_type_function_instance_type_pack(
        &mut self,
        _tp: TypePackId,
        _tfitp: &TypeFunctionInstanceTypePack,
    ) -> bool {
        LUAU_ASSERT!(false);
        false
    }
}
