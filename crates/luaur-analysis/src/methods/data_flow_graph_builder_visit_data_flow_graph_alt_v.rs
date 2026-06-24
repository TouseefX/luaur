use crate::enums::control_flow::ControlFlow;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::symbol::Symbol;
use luaur_ast::records::ast_stat_class::AstStatClass;
use luaur_common::FFlag;
use luaur_common::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn visit_ast_stat_class(&mut self, d: *mut AstStatClass) -> ControlFlow {
        LUAU_ASSERT!(FFlag::DebugLuauUserDefinedClasses.get());

        unsafe {
            let d = &*d;
            let symbol = Symbol::from_local(d.name);
            let def = (*self.def_arena).fresh_cell(symbol.clone(), (*d.name).location, false);

            *self.graph.local_defs.get_or_insert(d.name as *const _) = def;
            *(*self.current_scope())
                .bindings
                .get_or_insert(symbol.clone()) = def;
            self.captures.get_or_insert(symbol).all_versions.push(def);

            let members = &d.members;
            for i in 0..members.size as usize {
                let member = &*members.data.add(i);
                if let Some(prop) =
                    member.get_if::<luaur_ast::records::ast_class_property::AstClassProperty>()
                {
                    if !prop.ty.is_null() {
                        self.visit_type_ast_type(prop.ty);
                    }
                } else if let Some(method) =
                    member.get_if::<luaur_ast::records::ast_class_method::AstClassMethod>()
                {
                    self.visit_expr_ast_expr_function(method.function);
                }
            }
        }

        ControlFlow::None
    }
}
