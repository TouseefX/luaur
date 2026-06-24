//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:608-621`
//!
//! ```cpp
//! TypePackId deserialize(TypeFunctionTypePackId tp)
//! {
//!     shallowDeserialize(tp);
//!     run();
//!
//!     if (hasExceededIterationLimit() || hasErrors())
//!     {
//!         TypePackId error = state->ctx->builtins->errorTypePack;
//!         packs[tp] = error;
//!         return error;
//!     }
//!
//!     return find(tp).value_or(state->ctx->builtins->errorTypePack);
//! }
//! ```
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionDeserializer {
    pub fn deserialize_type_function_type_pack_id(
        &mut self,
        tp: TypeFunctionTypePackId,
    ) -> TypePackId {
        self.shallow_deserialize_type_function_type_pack_id(tp);
        self.run();

        if self.has_exceeded_iteration_limit() || self.has_errors() {
            let error: TypePackId =
                unsafe { (*(*(*self.state).ctx).builtins.as_ptr()).errorTypePack };
            *self.packs.get_or_insert(tp) = error;
            return error;
        }

        let error: TypePackId = unsafe { (*(*(*self.state).ctx).builtins.as_ptr()).errorTypePack };
        self.find_type_function_type_pack_id(tp).unwrap_or(error)
    }
}
