use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use crate::records::def_arena::DefArena;
use crate::records::dfg_scope::DfgScope;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_table::AstExprTable;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_table(&mut self, t: *mut AstExprTable) -> DataFlowResult {
        unsafe {
            let def_arena = self.def_arena;
            let table_cell =
                (*def_arena).fresh_cell(Symbol::default(), (*t).base.base.location, false);
            let scope = self.current_scope();
            // C++: currentScope()->props[tableCell] = {};
            *(*scope).props.get_or_insert(table_cell) = Default::default();

            let items = &(*t).items;
            for i in 0..items.size {
                let item = unsafe { &*items.data.add(i) };
                let result = self.visit_expr_ast_expr(item.value);
                if !item.key.is_null() {
                    self.visit_expr_ast_expr(item.key);
                    let key_node = item.key as *mut luaur_ast::records::ast_node::AstNode;
                    if (*key_node).is::<AstExprConstantString>() {
                        let string = &*(item.key as *mut AstExprConstantString);
                        let bytes = core::slice::from_raw_parts(
                            string.value.data as *const u8,
                            string.value.size,
                        );
                        let key_str = String::from_utf8_lossy(bytes).into_owned();
                        // C++: currentScope()->props[tableCell][string->value.data] = result.def;
                        let props = (*scope).props.get_or_insert(table_cell);
                        let def = result.def as *const crate::records::def::Def;
                        props.insert(key_str, def);
                    }
                }
            }

            // C++: return {tableCell, nullptr};
            DataFlowResult {
                def: table_cell as *const core::ffi::c_void,
                parent: core::ptr::null(),
            }
        }
    }
}
