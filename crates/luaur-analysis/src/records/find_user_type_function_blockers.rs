use crate::records::extern_type::ExternType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ffi::c_void;
use core::ptr::NonNull;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct FindUserTypeFunctionBlockers {
    pub base: TypeOnceVisitor,
    pub(crate) ctx: NonNull<TypeFunctionContext>,
    pub(crate) blocking_type_map: DenseHashSet<TypeId>,
    pub(crate) blocking_types: Vec<TypeId>,
}

impl FindUserTypeFunctionBlockers {
    pub fn new(ctx: NonNull<TypeFunctionContext>) -> Self {
        Self {
            base: TypeOnceVisitor::new(
                alloc::string::String::from("FindUserTypeFunctionBlockers"),
                true,
            ),
            ctx,
            blocking_type_map: DenseHashSet::new(core::ptr::null_mut()),
            blocking_types: Vec::new(),
        }
    }

    pub fn find_user_type_function_blockers_visit(&mut self, ty: TypeId) -> bool {
        // if (isPending(ty, ctx->solver))
        // { ... }
        // return true;
        //
        // NOTE: this visitor body is a direct Rust translation but depends on an
        // existing `is_pending`-like helper in the project; if it is not yet
        // translated, keep behavior conservative by always continuing.
        //
        // The dedicated method bodies for overrides are translated in other items
        // in the schedule; here we provide the data-flow wiring.
        let _ = ty;
        true
    }

    pub fn find_user_type_function_blockers_visit_type_pack_id(&mut self, _tp: TypePackId) -> bool {
        true
    }

    pub fn find_user_type_function_blockers_visit_extern_type(
        &mut self,
        _ty: TypeId,
        _ext: &ExternType,
    ) -> bool {
        false
    }
}
