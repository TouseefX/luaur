//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1108:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1108-1111` (hand-ported)

use crate::records::type_stringifier::TypeStringifier;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;

impl TypeStringifier {
    /// C++ `void operator()(TypeId, const UnknownType&)`.
    pub fn operator_call_21(&mut self, _ty: TypeId, _ttv: &UnknownType) {
        unsafe { (*self.state).emit("unknown") }
    }
}
