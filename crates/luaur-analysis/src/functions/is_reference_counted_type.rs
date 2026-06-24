use crate::enums::table_state::TableState;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::free_type::FreeType;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn is_reference_counted_type(typ: TypeId) -> bool {
    unsafe {
        if let Some(tt) = get_type_id::<TableType>(typ).as_ref() {
            tt.state == TableState::Free || tt.state == TableState::Unsealed
        } else {
            !get_type_id::<FreeType>(typ).is_null()
                || !get_type_id::<BlockedType>(typ).is_null()
                || !get_type_id::<PendingExpansionType>(typ).is_null()
        }
    }
}

#[allow(unused_imports)]
pub use is_reference_counted_type as is_reference_counted_type_type_id;
