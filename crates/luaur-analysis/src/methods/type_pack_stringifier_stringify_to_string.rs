//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1187:type_pack_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1187-1191` (hand-ported)

use crate::records::type_pack_stringifier::TypePackStringifier;
use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::type_id::TypeId;

impl TypePackStringifier {
    /// C++ `void stringify(TypeId tv)`.
    pub fn stringify_type_id(&mut self, tv: TypeId) {
        let mut tvs = TypeStringifier { state: self.state };
        tvs.stringify_type_id(tv);
    }
}
