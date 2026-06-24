//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:454:type_stringifier_stringify`
//! Source: `Analysis/src/ToString.cpp:454-500` (hand-ported)

use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::get_type_pack::get;
use crate::functions::is_empty::is_empty;
use crate::records::type_pack::TypePack;
use crate::records::type_stringifier::TypeStringifier;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl TypeStringifier {
    /// C++ `void stringify(const std::vector<TypeId>& types, const std::vector<TypePackId>& typePacks)`.
    pub fn stringify_vector_type_id_vector_type_pack_id(
        &mut self,
        types: &Vec<TypeId>,
        type_packs: &Vec<TypePackId>,
    ) {
        unsafe {
            if types.len() == 0 && type_packs.len() == 0 {
                return;
            }

            if types.len() > 0 || type_packs.len() > 0 {
                (*self.state).emit("<");
            }

            let mut first = true;

            for &ty in types.iter() {
                if !first {
                    (*self.state).emit(", ");
                }
                first = false;

                self.stringify_type_id(ty);
            }

            let single_tp = type_packs.len() == 1;

            for &tp in type_packs.iter() {
                if is_empty(tp) && single_tp {
                    continue;
                }

                if !first {
                    (*self.state).emit(", ");
                } else {
                    first = false;
                }

                let mut wrap = !single_tp && !get::<TypePack>(follow_type_pack_id(tp)).is_null();

                wrap &= !is_empty(tp);

                if wrap {
                    (*self.state).emit("(");
                }

                self.stringify_type_pack_id(tp);

                if wrap {
                    (*self.state).emit(")");
                }
            }

            if types.len() > 0 || type_packs.len() > 0 {
                (*self.state).emit(">");
            }
        }
    }
}
