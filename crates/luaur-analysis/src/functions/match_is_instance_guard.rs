use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::refinement_key::RefinementKey;
use core::ffi::CStr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::rtti::ast_node_as;

pub fn match_is_instance_guard(call: &AstExprCall, dfg: &DataFlowGraph) -> *const RefinementKey {
    unsafe {
        let index = ast_node_as::<AstExprIndexName>(call.func as *mut _);
        if index.is_null() || (*index).op != '.' as i8 {
            return core::ptr::null();
        }

        if (*index).index.value.is_null()
            || CStr::from_ptr((*index).index.value).to_bytes() != b"isinstance"
        {
            return core::ptr::null();
        }

        if ast_node_as::<AstExprGlobal>((*index).expr as *mut _).is_null() {
            return core::ptr::null();
        }

        if call.args.size < 1 {
            return core::ptr::null();
        }

        dfg.get_refinement_key(*call.args.data)
    }
}
