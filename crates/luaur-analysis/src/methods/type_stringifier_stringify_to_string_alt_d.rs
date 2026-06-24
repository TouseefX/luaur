//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1385:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:1385-1389` (hand-ported)

use crate::records::type_pack_stringifier::TypePackStringifier;
use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeStringifier {
    /// C++ `void TypeStringifier::stringify(TypePackId tp)`.
    pub fn stringify_type_pack_id(&mut self, tp: TypePackId) {
        let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state(self.state);
        tps.stringify_type_pack_id(tp);
    }
}
