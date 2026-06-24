//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:913-921`
//!
//! ```cpp
//! void deserializeChildren(TypeFunctionKind tfkind, TypeOrPack kind)
//! {
//!     if (auto [ty, tfty] = std::tuple{get<TypeId>(kind), get<TypeFunctionTypeId>(tfkind)}; ty && tfty)
//!         deserializeChildren(*tfty, *ty);
//!     else if (auto [tp, tftp] = std::tuple{get<TypePackId>(kind), get<TypeFunctionTypePackId>(tfkind)}; tp && tftp)
//!         deserializeChildren(*tftp, *tp);
//!     else
//!         state->ctx->ice->ice("Deserializing user defined type function arguments: tfkind and kind do not represent the same type");
//! }
//! ```
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::type_aliases::type_function_kind::{TypeFunctionKind, TypeFunctionKindMember};
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::{TypeOrPack, TypeOrPackMember};
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_kind_type_or_pack(
        &mut self,
        tfkind: TypeFunctionKind,
        kind: TypeOrPack,
    ) {
        if let Some(ty) = <TypeId as TypeOrPackMember>::get_if(&kind) {
            if let Some(tfty) = <TypeFunctionTypeId as TypeFunctionKindMember>::get_if(&tfkind) {
                self.deserialize_children_type_function_type_id_type_id(*tfty, *ty);
                return;
            }
        }

        if let Some(tp) = <TypePackId as TypeOrPackMember>::get_if(&kind) {
            if let Some(tftp) = <TypeFunctionTypePackId as TypeFunctionKindMember>::get_if(&tfkind)
            {
                self.deserialize_children_type_function_type_pack_id_type_pack_id(*tftp, *tp);
                return;
            }
        }

        unsafe {
            (*(*self.state).ctx)
                .ice
                .as_ref()
                .ice_string("Deserializing user defined type function arguments: tfkind and kind do not represent the same type");
        }
    }
}
