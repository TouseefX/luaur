use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::type_aliases::type_function_kind::{TypeFunctionKind, TypeFunctionKindMember};
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionCloner {
    pub fn find_type_function_kind(&self, kind: TypeFunctionKind) -> Option<TypeFunctionKind> {
        if let Some(ty) = <TypeFunctionTypeId as TypeFunctionKindMember>::get_if(&kind) {
            self.find_type_function_type_id(*ty)
                .map(TypeFunctionKind::V0)
        } else if let Some(tp) = <TypeFunctionTypePackId as TypeFunctionKindMember>::get_if(&kind) {
            self.find_type_function_type_pack_id(*tp)
                .map(TypeFunctionKind::V1)
        } else {
            LUAU_ASSERT!(false);
            None
        }
    }
}
