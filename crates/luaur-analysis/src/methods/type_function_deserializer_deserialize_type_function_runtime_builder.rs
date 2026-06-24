//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:593-606`
//!
//! ```cpp
//! TypeId deserialize(TypeFunctionTypeId ty)
//! {
//!     shallowDeserialize(ty);
//!     run();
//!
//!     if (hasExceededIterationLimit() || hasErrors())
//!     {
//!         TypeId error = state->ctx->builtins->errorType;
//!         types[ty] = error;
//!         return error;
//!     }
//!
//!     return find(ty).value_or(state->ctx->builtins->errorType);
//! }
//! ```
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

impl TypeFunctionDeserializer {
    pub fn deserialize_type_function_type_id(&mut self, ty: TypeFunctionTypeId) -> TypeId {
        self.shallow_deserialize_type_function_type_id(ty);
        self.run();

        if self.has_exceeded_iteration_limit() || self.has_errors() {
            let error: TypeId = unsafe { (*(*(*self.state).ctx).builtins.as_ptr()).errorType };
            *self.types.get_or_insert(ty) = error;
            return error;
        }

        let error: TypeId = unsafe { (*(*(*self.state).ctx).builtins.as_ptr()).errorType };
        self.find_type_function_type_id(ty).unwrap_or(error)
    }
}
