use crate::enums::table_state::TableState;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::subsumes_scope::subsumes;
use crate::records::scope::Scope;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn seal_table(scope: *mut Scope, ty: TypeId) {
    unsafe {
        let follow_ty = follow_type_id(ty);
        let table_ty = get_mutable_type_id::<TableType>(follow_ty);

        if table_ty.is_null() {
            return;
        }

        if !subsumes(scope, (*table_ty).scope) {
            return;
        }

        if (*table_ty).state == TableState::Unsealed || (*table_ty).state == TableState::Free {
            (*table_ty).state = TableState::Sealed;
        }
    }
}

#[allow(unused_imports)]
pub use seal_table as seal_table_scope_type_id;
