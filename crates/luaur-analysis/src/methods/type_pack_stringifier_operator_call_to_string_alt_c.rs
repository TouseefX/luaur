//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1288:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1288-1296` (hand-ported)

use crate::records::type_pack_stringifier::TypePackStringifier;
use crate::records::variadic_type_pack::VariadicTypePack;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypePackStringifier {
    /// C++ `void operator()(TypePackId, const VariadicTypePack& pack)`.
    pub fn operator_call_8(&mut self, _id: TypePackId, pack: &VariadicTypePack) {
        unsafe {
            (*self.state).emit("...");
            if luaur_common::FInt::DebugLuauVerboseTypeNames.get() >= 1 && pack.hidden {
                (*self.state).emit("*hidden*");
            }
            self.stringify_type_id(pack.ty);
        }
    }
}
