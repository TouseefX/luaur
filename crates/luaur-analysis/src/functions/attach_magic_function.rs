use crate::functions::get_mutable_constraint::get_mutable_constraint;
use crate::functions::get_mutable_txn_log::get_mutable_pending_type;
use crate::functions::get_mutable_txn_log_alt_c::get_mutable_pending_type_pack;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_mutable_type_function_runtime::get_mutable_type_function_singleton_type;
use crate::functions::get_mutable_type_function_runtime_alt_f::get_mutable_type_function_type_pack_id;
use crate::functions::get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id;
use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::functions::get_mutable_type_utils::get_mutable_optional_ty;
use crate::records::function_type::FunctionType;
use crate::records::magic_function::MagicFunction;
use crate::type_aliases::type_id::TypeId;
use alloc::sync::Arc;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn attach_magic_function(ty: TypeId, magic: Arc<MagicFunction>) {
    unsafe {
        let ftv = get_mutable_type_id::<FunctionType>(ty);
        if !ftv.is_null() {
            (*ftv).magic = Some(magic);
        } else {
            LUAU_ASSERT!(false);
        }
    }
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use attach_magic_function as attach_magic_function_TypeId_Arc_MagicFunction;
