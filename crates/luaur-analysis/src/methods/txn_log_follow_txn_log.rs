//! @interface-stub
use crate::enums::follow_option::FollowOption;
use crate::functions::follow_type_alt_e::follow_full;
use crate::records::r#type::Type;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;

fn pending_type_mapper(context: *const core::ffi::c_void, ty: TypeId) -> TypeId {
    let log = unsafe { &*(context as *const TxnLog) };
    let state = log.pending_type_id(ty);

    if state.is_null() {
        ty
    } else {
        unsafe { &(*state).pending as *const Type }
    }
}

impl TxnLog {
    pub fn follow_type_id(&self, ty: TypeId) -> TypeId {
        unsafe {
            follow_full(
                ty,
                FollowOption::Normal,
                self as *const TxnLog as *const core::ffi::c_void,
                pending_type_mapper,
            )
        }
    }
}
