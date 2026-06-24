use crate::records::type_function_cloner::TypeFunctionCloner;
use crate::type_aliases::type_function_kind::{TypeFunctionKind, TypeFunctionKindMember};
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeFunctionCloner {
    pub fn clone_children_type_function_kind_type_function_kind(
        &mut self,
        kind: &TypeFunctionKind,
        tfkind: &TypeFunctionKind,
    ) {
        if let Some(ty) = TypeFunctionKind::get_if::<TypeFunctionTypeId>(kind) {
            if let Some(tfty) = TypeFunctionKind::get_if::<TypeFunctionTypeId>(tfkind) {
                self.clone_children_type_function_type_id_type_function_type_id(*ty, *tfty);
                return;
            }
        }

        if let Some(tp) = TypeFunctionKind::get_if::<TypeFunctionTypePackId>(kind) {
            if let Some(tftp) = TypeFunctionKind::get_if::<TypeFunctionTypePackId>(tfkind) {
                self.clone_children_type_function_type_pack_id_type_function_type_pack_id(
                    *tp, *tftp,
                );
                return;
            }
        }

        LUAU_ASSERT!(false);
    }
}
