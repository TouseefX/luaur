use crate::functions::contains_subscripted_definition::contains_subscripted_definition;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::def_arena::DefArena;
use crate::records::dfg_scope::DfgScope;
use crate::type_aliases::def_id_def::DefId;
use alloc::string::String;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl DataFlowGraphBuilder {
    pub fn visit_l_value_ast_expr_index_name_def_id(
        &mut self,
        i: *mut AstExprIndexName,
        incoming_def: DefId,
    ) -> DefId {
        // C++:
        //   DefId parentDef = visitExpr(i->expr).def;
        //   DfgScope* scope = currentScope();
        //   DefId updated = defArena->freshCell(i->index, i->location, containsSubscriptedDefinition(incomingDef));
        //   scope->props[parentDef][i->index.value] = updated;
        //   return updated;
        unsafe {
            let parent_def = self.visit_expr_ast_expr((*i).expr).def as DefId;
            let scope = self.current_scope();
            let index_str =
                String::from(core::ffi::CStr::from_ptr((*i).index.value).to_string_lossy());
            let subscripted = contains_subscripted_definition(incoming_def);
            let updated = (*self.def_arena).fresh_cell(
                crate::records::symbol::Symbol::from_global((*i).index),
                (*i).base.base.location,
                subscripted,
            );
            (*scope)
                .props
                .get_or_insert(parent_def)
                .insert(index_str, updated);
            updated
        }
    }
}
