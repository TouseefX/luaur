//! Source: `Analysis/src/ConstraintGenerator.cpp:3182-3203` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprLocal* local)`.
use crate::functions::follow_type::follow;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_local(
        &mut self,
        scope: &ScopePtr,
        local: *mut AstExprLocal,
    ) -> Inference {
        unsafe {
            let key = (*self.dfg).get_refinement_key(local as *const AstExpr);
            LUAU_ASSERT!(!key.is_null());

            let mut maybe_ty: Option<TypeId> = None;

            // if we have a refinement key, we can look up its type.
            if !key.is_null() {
                // C++ default `prototype = true`.
                maybe_ty = self.lookup(
                    scope,
                    (*local).base.base.location,
                    (*key).def as crate::type_aliases::def_id_def::DefId,
                    true,
                );
            }

            if let Some(ty) = maybe_ty {
                let ty = follow(ty);

                self.record_inferred_binding((*local).local, ty);

                let refinement = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(key, (*self.builtin_types).truthyType);
                Inference::inference_type_id_refinement_id(ty, refinement)
            } else {
                (*self.ice).ice_string("CG: AstExprLocal came before its declaration?");
                Inference::inference_type_id_refinement_id(
                    (*self.builtin_types).errorType,
                    core::ptr::null_mut(),
                )
            }
        }
    }
}
