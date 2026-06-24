use crate::records::iterative_type_function_type_visitor::IterativeTypeFunctionTypeVisitor;
use crate::records::work_item_iterative_type_function_type_visitor::WorkItem;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;

impl IterativeTypeFunctionTypeVisitor {
    pub fn work_item_type_function_type_pack_id_i32(
        tp: TypeFunctionTypePackId,
        parent: i32,
    ) -> WorkItem {
        WorkItem {
            t: tp as *const core::ffi::c_void,
            is_type: false,
            parent,
        }
    }
}
