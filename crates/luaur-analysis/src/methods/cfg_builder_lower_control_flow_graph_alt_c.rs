use crate::records::cfg_builder::CfgBuilder;
use crate::records::declare::Declare;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl CfgBuilder {
    pub fn lower_ast_stat_local(&mut self, local: *mut AstStatLocal) {
        unsafe {
            let local_ref = &*local;
            let vars_size = local_ref.vars.size;
            let values_size = local_ref.values.size;

            for i in 0..vars_size {
                let loc = *local_ref.vars.data.add(i);
                let expr = if i < values_size {
                    *local_ref.values.data.add(i)
                } else {
                    core::ptr::null_mut()
                };

                if !expr.is_null() {
                    self.lower_expr_ast_expr(expr);
                }

                // C++:
                //   Symbol sym(loc);
                //   DefId def = newDefinition(sym);
                //   emit<Declare>(currentBlock, def, local);
                //   currentBlock->setReachingDefinition(sym, def);
                let sym = Symbol::from_local(loc);
                let def: DefId = self.new_definition(sym.clone());
                let current_block = self.current_block;
                self.emit::<Declare, _>(current_block, (def, local));
                (*current_block).set_reaching_definition(sym, def);
            }
        }
    }
}
