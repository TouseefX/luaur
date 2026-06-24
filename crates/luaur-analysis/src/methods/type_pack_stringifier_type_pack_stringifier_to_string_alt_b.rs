//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1181:type_pack_stringifier_type_pack_stringifier`
//! Source: `Analysis/src/ToString.cpp:1181-1184` (hand-ported)

use crate::records::stringifier_state::StringifierState;
use crate::records::type_pack_stringifier::TypePackStringifier;
use alloc::vec::Vec;

impl TypePackStringifier {
    /// C++ `explicit TypePackStringifier(StringifierState& state)` — uses the
    /// empty `dummyElemNames`.
    pub fn type_pack_stringifier_stringifier_state(state: *mut StringifierState) -> Self {
        Self {
            state,
            elem_names: Vec::new(),
            elem_index: 0,
        }
    }
}
