use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::def::Def;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use alloc::string::String;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_node::AstNode;

fn refinement_def_id(def: DefId) -> crate::type_aliases::def_id_refinement::DefId {
    core::ptr::NonNull::new(def as *mut *const Def).unwrap()
}

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_index_expr(&mut self, i: *mut AstExprIndexExpr) -> DataFlowResult {
        unsafe {
            let parent = self.visit_expr_ast_expr((*i).expr);
            self.visit_expr_ast_expr((*i).index);

            let index_node = (*i).index as *mut AstNode;
            if (*index_node).is::<AstExprConstantString>() {
                let string = &*((*i).index as *mut AstExprConstantString);
                let bytes =
                    core::slice::from_raw_parts(string.value.data as *const u8, string.value.size);
                let index = String::from_utf8_lossy(bytes).into_owned();

                let def = self.lookup_def_id_string_location(
                    parent.def as *const Def,
                    &index,
                    (*i).base.base.location,
                );
                let key = (*self.key_arena).node(parent.parent, refinement_def_id(def), &index);
                return DataFlowResult {
                    def: def as *const core::ffi::c_void,
                    parent: key,
                };
            }

            let def =
                (*self.def_arena).fresh_cell(Symbol::default(), (*i).base.base.location, true);
            DataFlowResult {
                def: def as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
