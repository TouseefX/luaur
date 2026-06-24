use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::def::Def;
use crate::type_aliases::def_id_def::DefId;
use alloc::string::String;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

fn refinement_def_id(def: DefId) -> crate::type_aliases::def_id_refinement::DefId {
    core::ptr::NonNull::new(def as *mut *const Def).unwrap()
}

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_index_name(&mut self, i: *mut AstExprIndexName) -> DataFlowResult {
        unsafe {
            let parent = self.visit_expr_ast_expr((*i).expr);
            let index = String::from(core::ffi::CStr::from_ptr((*i).index.value).to_string_lossy());
            let def = self.lookup_def_id_string_location(
                parent.def as *const Def,
                &index,
                (*i).base.base.location,
            );
            let key = (*self.key_arena).node(parent.parent, refinement_def_id(def), &index);

            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: key,
            }
        }
    }
}
