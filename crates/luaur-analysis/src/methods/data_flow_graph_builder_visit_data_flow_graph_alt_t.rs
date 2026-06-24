use crate::enums::control_flow::ControlFlow;
use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::push_scope::PushScope;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_declare_function::AstStatDeclareFunction;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_declare_function(
        &mut self,
        d: *mut AstStatDeclareFunction,
    ) -> ControlFlow {
        unsafe {
            let symbol = Symbol::from_global((*d).name);
            let def = (*self.def_arena).fresh_cell(symbol.clone(), (*d).name_location, false);
            *self.graph.declared_defs.get_or_insert(d as *const AstStat) = def;
            *(*self.current_scope())
                .bindings
                .get_or_insert(symbol.clone()) = def;
            self.captures.get_or_insert(symbol).all_versions.push(def);

            let unreachable: *mut DfgScope = self.make_child_scope(ScopeType::Linear);
            let _ps = PushScope::push_scope(&mut self.scope_stack, unreachable);

            self.visit_generics((*d).generics);
            self.visit_generic_packs((*d).generic_packs);
            self.visit_type_list((*d).params.clone());
            self.visit_type_pack_ast_type_pack((*d).ret_types);
        }

        ControlFlow::None
    }
}
