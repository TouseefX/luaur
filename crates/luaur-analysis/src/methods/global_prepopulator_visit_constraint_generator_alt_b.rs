use crate::records::binding::Binding;
use crate::records::blocked_type::BlockedType;
use crate::records::global_prepopulator::GlobalPrepopulator;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::rtti::ast_node_as;

impl GlobalPrepopulator {
    pub fn visit_ast_stat_assign(&mut self, assign: *mut AstStatAssign) -> bool {
        unsafe {
            let vars = (*assign).vars;
            for i in 0..vars.size {
                let expr = *vars.data.add(i);
                if expr.is_null() {
                    continue;
                }

                let global = ast_node_as::<AstExprGlobal>(expr as *mut AstNode);
                if global.is_null() {
                    continue;
                }

                let name = (*global).name;
                let sym = Symbol::from_global(name);

                let scope: &mut Scope = self.global_scope.as_mut();

                // if (!globalScope->lookup(g->name)) globalScope->globalsToWarn.insert(g->name.value)
                if scope.lookup_symbol(sym.clone()).is_none() {
                    let name_str = core::ffi::CStr::from_ptr(name.value)
                        .to_string_lossy()
                        .into_owned();
                    scope.globals_to_warn.insert(name_str);
                }

                if scope.bindings.contains_key(&sym) {
                    continue;
                }

                // TypeId bt = arena->addType(BlockedType{})
                let bt_ty: TypeId = (*self.arena.as_ptr()).add_type(BlockedType::default());
                self.uninitialized_globals.insert(name);

                // globalScope->bindings[g->name] = Binding{bt, g->location}
                let binding = Binding {
                    type_id: bt_ty,
                    location: (*expr).base.location,
                    deprecated: false,
                    deprecated_suggestion: alloc::string::String::new(),
                    documentation_symbol: None,
                };

                scope.bindings.insert(sym, binding);
            }
        }
        true
    }
}
