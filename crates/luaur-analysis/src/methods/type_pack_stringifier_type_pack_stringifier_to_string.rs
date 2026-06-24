//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1175:type_pack_stringifier_type_pack_stringifier`
//! Source: `Analysis/src/ToString.cpp:1175-1179` (hand-ported)

use crate::records::function_argument::FunctionArgument;
use crate::records::stringifier_state::StringifierState;
use crate::records::type_pack_stringifier::TypePackStringifier;
use alloc::vec::Vec;

impl TypePackStringifier {
    /// C++ `explicit TypePackStringifier(StringifierState& state, const std::vector<std::optional<FunctionArgument>>& elemNames)`.
    pub fn type_pack_stringifier_stringifier_state_vector_optional_function_argument(
        state: *mut StringifierState,
        elem_names: &Vec<Option<FunctionArgument>>,
    ) -> Self {
        Self {
            state,
            elem_names: elem_names.clone(),
            elem_index: 0,
        }
    }
}
