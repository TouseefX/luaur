//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:1118:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:1118-1133` (hand-ported)

use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get;
use crate::records::intersection_type::IntersectionType;
use crate::records::negation_type::NegationType;
use crate::records::type_stringifier::TypeStringifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl TypeStringifier {
    /// C++ `void operator()(TypeId, const NegationType& ntv)`.
    pub fn operator_call_14(&mut self, _ty: TypeId, ntv: &NegationType) {
        unsafe {
            (*self.state).emit("~");

            // The precedence of `~` should be less than `|` and `&`.
            let followed = follow(ntv.ty);
            let parens = !get::<UnionType>(followed).is_null()
                || !get::<IntersectionType>(followed).is_null();

            if parens {
                (*self.state).emit("(");
            }

            self.stringify_type_id(ntv.ty);

            if parens {
                (*self.state).emit(")");
            }
        }
    }
}
