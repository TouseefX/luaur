use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::module::Module;
use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_node::AstNode;

impl ConstraintGenerator {
    pub fn child_scope(&mut self, node: *mut AstNode, parent: &ScopePtr) -> ScopePtr {
        let scope: ScopePtr = alloc::sync::Arc::new(Scope::new(parent, 0));
        let scope_raw = scope.as_ref() as *const Scope as *mut Scope;
        self.scopes
            .push((unsafe { (*node).location }, scope.clone()));

        unsafe {
            (*scope_raw).location = (*node).location;
            (*scope_raw).return_type = parent.return_type;
            (*scope_raw).vararg_pack = parent.vararg_pack;

            let parent_raw = parent.as_ref() as *const Scope as *mut Scope;
            (*parent_raw).children.push(scope_raw);
        }

        if let Some(module) = &self.module {
            let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
            unsafe {
                *(*module_ptr)
                    .ast_scopes
                    .get_or_insert(node as *const AstNode) = scope_raw;
            }
        }

        scope
    }
}
