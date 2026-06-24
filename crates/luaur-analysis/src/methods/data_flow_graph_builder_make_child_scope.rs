use crate::enums::scope_type::ScopeType;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl DataFlowGraphBuilder {
    pub fn make_child_scope(&mut self, scope_type: ScopeType) -> *mut DfgScope {
        let parent_scope = self.current_scope();
        // C++ `new DfgScope{currentScope(), scopeType}` uses default member
        // inits: `bindings{Symbol{}}`, `props{nullptr}`.
        let new_scope = DfgScope {
            parent: parent_scope,
            scope_type,
            bindings: crate::type_aliases::bindings::Bindings::new(
                crate::records::symbol::Symbol::default(),
            ),
            props: crate::type_aliases::props_data_flow_graph::Props::new(core::ptr::null()),
        };
        let mut boxed = alloc::boxed::Box::new(new_scope);
        let ptr = boxed.as_mut() as *mut DfgScope;
        self.scopes.push(boxed);
        ptr
    }
}
