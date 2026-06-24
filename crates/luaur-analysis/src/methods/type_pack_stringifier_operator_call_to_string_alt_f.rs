//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1345:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1345-1348` (hand-ported)

use crate::records::type_pack_stringifier::TypePackStringifier;
use crate::type_aliases::bound_type_pack::BoundTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypePackStringifier {
    /// C++ `void operator()(TypePackId, const BoundTypePack& btv)`.
    pub fn operator_call_4(&mut self, _id: TypePackId, btv: &BoundTypePack) {
        self.stringify_type_pack_id(btv.boundTo);
    }
}
