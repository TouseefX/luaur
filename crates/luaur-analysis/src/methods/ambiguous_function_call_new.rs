use crate::records::ambiguous_function_call::AmbiguousFunctionCall;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl AmbiguousFunctionCall {
    pub fn new(function: TypeId, arguments: TypePackId) -> Self {
        Self {
            function,
            arguments,
        }
    }
}
