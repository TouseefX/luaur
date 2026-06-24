//! Source: `Analysis/src/ConstraintGenerator.cpp:3205-3221` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprGlobal* global)`.
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_global(
        &mut self,
        scope: &ScopePtr,
        global: *mut AstExprGlobal,
    ) -> Inference {
        unsafe {
            let key = (*self.dfg).get_refinement_key(global as *const AstExpr);
            LUAU_ASSERT!(!key.is_null());

            let def = (*key).def as crate::type_aliases::def_id_def::DefId;

            // prepopulateGlobalScope() has already added all global functions to the environment by this point, so any
            // global that is not already in-scope is definitely an unknown symbol.
            if let Some(ty) = self.lookup(
                scope,
                (*global).base.base.location,
                def,
                /*prototype=*/ false,
            ) {
                let refinement = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(key, (*self.builtin_types).truthyType);
                Inference::inference_type_id_refinement_id(ty, refinement)
            } else {
                Inference::inference_type_id_refinement_id(
                    (*self.builtin_types).errorType,
                    core::ptr::null_mut(),
                )
            }
        }
    }
}
