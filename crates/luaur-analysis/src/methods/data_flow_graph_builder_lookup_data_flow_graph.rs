use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::dfg_scope::DfgScope;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_def::DefId;
use luaur_ast::records::location::Location;

impl DataFlowGraphBuilder {
    pub fn lookup_symbol_location(&mut self, symbol: Symbol, location: Location) -> DefId {
        let scope = self.current_scope();

        let mut outside_loop_scope = false;
        let mut current: *mut DfgScope = scope;
        while !current.is_null() {
            unsafe {
                outside_loop_scope = outside_loop_scope
                    || (*current).scope_type == crate::enums::scope_type::ScopeType::Loop;

                if let Some(found) = (*current).bindings.find(&symbol) {
                    return *found;
                } else if (*current).scope_type == crate::enums::scope_type::ScopeType::Function {
                    let capture = self.captures.get_or_insert(symbol.clone());
                    let capture_def = (*self.def_arena).phi_vector_def_id(&alloc::vec::Vec::new());
                    capture.capture_defs.push(capture_def);

                    if !outside_loop_scope {
                        *(*scope).bindings.get_or_insert(symbol.clone()) = capture_def;
                    }

                    return capture_def;
                }
            }

            unsafe {
                current = (*current).parent;
            }
        }

        unsafe {
            let result = (*self.def_arena).fresh_cell(symbol.clone(), location, false);
            *(*scope).bindings.get_or_insert(symbol.clone()) = result;
            self.captures
                .get_or_insert(symbol)
                .all_versions
                .push(result);
            result
        }
    }
}
