//! Source: `Analysis/src/SubtypingUnifier.cpp:24-31` — `SubtypingUnifier::canBeUnified`.

use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_blocked_type_utils::is_blocked;
use crate::records::free_type::FreeType;
use crate::records::subtyping_unifier::SubtypingUnifier;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

impl SubtypingUnifier {
    pub fn can_be_unified(&self, ty: TypeId) -> bool {
        let ty = unsafe { follow_type_id(ty) };
        let tbl = unsafe { get_type_id::<TableType>(ty) };
        if !tbl.is_null() {
            return unsafe { (*tbl).state } != TableState::Sealed;
        }

        !unsafe { get_type_id::<FreeType>(ty) }.is_null() || is_blocked(ty)
    }
}
