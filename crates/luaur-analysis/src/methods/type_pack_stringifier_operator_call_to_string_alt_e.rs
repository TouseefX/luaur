//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1326:type_pack_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1326-1343` (hand-ported)

use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack_stringifier::TypePackStringifier;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypePackStringifier {
    /// C++ `void operator()(TypePackId tp, const FreeTypePack& pack)`.
    pub fn operator_call(&mut self, tp: TypePackId, pack: &FreeTypePack) {
        unsafe {
            (*(*self.state).result).invalid = true;
            if luaur_common::FInt::DebugLuauVerboseTypeNames.get() >= 1 {
                (*self.state).emit("free-");
            }
            let name = (*self.state).get_name_type_pack_id(tp);
            (*self.state).emit(name.as_str());

            if luaur_common::FInt::DebugLuauVerboseTypeNames.get() >= 1 {
                (*self.state).emit_polarity(pack.polarity);
            }

            if luaur_common::FInt::DebugLuauVerboseTypeNames.get() >= 2 {
                (*self.state).emit("-");
                (*self.state).emit_level(pack.scope);
            }

            (*self.state).emit("...");
        }
    }
}
