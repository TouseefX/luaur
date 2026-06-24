use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::global_prepopulator::GlobalPrepopulator;
use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use core::ptr::NonNull;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::visit::AstVisitable;
use luaur_common::records::dense_hash_set::DenseHashSet;

impl ConstraintGenerator {
    // ConstraintGenerator::prepopulateGlobalScopeForFragmentTypecheck(
    //     const ScopePtr&, const ScopePtr&, AstStatBlock*) (ConstraintGenerator.cpp:4957).
    pub fn prepopulate_global_scope_for_fragment_typecheck(
        &mut self,
        _global_scope: &ScopePtr,
        _resume_scope: &ScopePtr,
        program: *mut AstStatBlock,
    ) {
        // Handle type function globals as well, without preparing a module scope since
        // they have a separate environment.
        let root_scope_raw = unsafe {
            (*self.type_function_runtime).root_scope.as_ref() as *const Scope as *mut Scope
        };

        let mut tfgp = GlobalPrepopulator {
            global_scope: unsafe { NonNull::new_unchecked(root_scope_raw) },
            arena: unsafe { NonNull::new_unchecked(self.arena) },
            dfg: unsafe { NonNull::new_unchecked(self.dfg as *mut _) },
            uninitialized_globals: DenseHashSet::new(AstName {
                value: core::ptr::null(),
            }),
        };

        unsafe {
            (*program).visit(&mut tfgp);
        }

        for name in tfgp.uninitialized_globals.iter() {
            self.uninitialized_globals.insert(name);
        }
    }
}
