use crate::records::find_user_type_function_blockers::FindUserTypeFunctionBlockers;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl FindUserTypeFunctionBlockers {
    pub fn find_user_type_function_blockers(ctx: core::ptr::NonNull<TypeFunctionContext>) -> Self {
        FindUserTypeFunctionBlockers {
            base: TypeOnceVisitor::new("FindUserTypeFunctionBlockers".to_string(), true),
            ctx,
            blocking_type_map: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null_mut(),
            ),
            blocking_types: alloc::vec::Vec::new(),
        }
    }
}
