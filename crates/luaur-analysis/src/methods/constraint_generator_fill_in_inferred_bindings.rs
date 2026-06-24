//! @interface-stub
use crate::records::binding::Binding;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn fill_in_inferred_bindings(
        &mut self,
        _global_scope: &ScopePtr,
        _block: *mut AstStatBlock,
    ) {
        let inferred_bindings: Vec<(Symbol, *mut Scope, Location, Vec<TypeId>)> = self
            .inferred_bindings
            .iter()
            .map(|(symbol, p)| (symbol.clone(), p.scope, p.location, p.types.order.clone()))
            .collect();

        for (symbol, scope, location, tys) in inferred_bindings {
            let ty = if tys.len() == 1 {
                tys[0]
            } else {
                self.make_union_vector_type_id(tys)
            };

            unsafe {
                (*scope).bindings.insert(
                    symbol,
                    Binding {
                        type_id: ty,
                        location,
                        deprecated: false,
                        deprecated_suggestion: String::new(),
                        documentation_symbol: None,
                    },
                );
            }
        }
    }
}
