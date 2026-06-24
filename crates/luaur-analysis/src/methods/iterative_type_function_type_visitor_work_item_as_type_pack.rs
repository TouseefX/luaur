use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;

impl IterativeTypeFunctionTypeVisitor {
    pub fn work_item_as_type_pack(&self) -> Option<*const TypeFunctionTypePackId> {
        Some(core::ptr::null_mut())
    }
}
