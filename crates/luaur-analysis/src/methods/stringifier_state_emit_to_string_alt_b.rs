//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:280:stringifier_state_emit`
//! Source: `Analysis/src/ToString.cpp:280-285` (hand-ported)

use crate::records::stringifier_state::StringifierState;
use crate::records::type_level::TypeLevel;
use alloc::string::ToString;

impl StringifierState {
    /// C++ `void emit(TypeLevel level)`.
    pub fn emit_type_level(&mut self, level: TypeLevel) {
        self.emit_string(&level.level.to_string());
        self.emit_string("-");
        self.emit_string(&level.subLevel.to_string());
    }
}
