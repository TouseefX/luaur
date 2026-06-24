//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:305:stringifier_state_emit_and_record_span`
//! Source: `Analysis/src/ToString.cpp:305-313` (hand-ported)

use crate::records::stringifier_state::StringifierState;
use crate::records::to_string_span::ToStringSpan;
use crate::type_aliases::type_id::TypeId;

impl StringifierState {
    /// C++ `void emitAndRecordSpan(const std::string& s, TypeId ty)`.
    pub fn emit_and_record_span(&mut self, s: &str, ty: TypeId) {
        unsafe {
            let start_pos = (&(*self.result).name).len();
            self.emit(s);
            let end_pos = (&(*self.result).name).len();

            if end_pos > start_pos {
                (*self.result).type_spans.push(ToStringSpan {
                    start_pos,
                    end_pos,
                    r#type: ty,
                });
            }
        }
    }
}
