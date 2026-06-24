//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:502:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:502-540` (hand-ported)

use crate::functions::follow_type::follow;
use crate::functions::get_type_alt_j::get;
use crate::records::free_type::FreeType;
use crate::records::never_type::NeverType;
use crate::records::type_stringifier::TypeStringifier;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypeStringifier {
    /// C++ `void operator()(TypeId ty, const FreeType& ftv)`.
    pub fn operator_call_2(&mut self, ty: TypeId, ftv: &FreeType) {
        unsafe {
            (*(*self.state).result).invalid = true;

            // Free types are guaranteed to have upper and lower bounds now.
            LUAU_ASSERT!(!ftv.lower_bound.is_null());
            LUAU_ASSERT!(!ftv.upper_bound.is_null());
            let lower_bound = follow(ftv.lower_bound);
            let upper_bound = follow(ftv.upper_bound);
            if !get::<NeverType>(lower_bound).is_null()
                && !get::<UnknownType>(upper_bound).is_null()
            {
                (*self.state).emit("'");
                let name = (*self.state).get_name_type_id(ty);
                (*self.state).emit(name.as_str());
                if luaur_common::FInt::DebugLuauVerboseTypeNames.get() >= 1 {
                    (*self.state).emit_polarity(ftv.polarity);
                }
            } else {
                (*self.state).emit("(");
                if get::<NeverType>(lower_bound).is_null() {
                    self.stringify_type_id(lower_bound);
                    (*self.state).emit(" <: ");
                }
                (*self.state).emit("'");
                let name = (*self.state).get_name_type_id(ty);
                (*self.state).emit(name.as_str());

                if luaur_common::FInt::DebugLuauVerboseTypeNames.get() >= 1 {
                    (*self.state).emit_polarity(ftv.polarity);
                }

                if get::<UnknownType>(upper_bound).is_null() {
                    (*self.state).emit(" <: ");
                    self.stringify_type_id(upper_bound);
                }
                (*self.state).emit(")");
            }
        }
    }
}
