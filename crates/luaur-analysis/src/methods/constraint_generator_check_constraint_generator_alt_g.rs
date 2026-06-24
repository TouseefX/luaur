//! Source: `Analysis/src/ConstraintGenerator.cpp:3287-3316` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprIndexExpr* indexExpr)`.
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::has_indexer_constraint::HasIndexerConstraint;
use crate::records::inference::Inference;
use crate::records::module::Module;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use alloc::string::String;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::rtti::ast_node_as;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_index_expr(
        &mut self,
        scope: &ScopePtr,
        index_expr: *mut AstExprIndexExpr,
    ) -> Inference {
        unsafe {
            let constant_string =
                ast_node_as::<AstExprConstantString>((*index_expr).index as *mut AstNode);
            if !constant_string.is_null() {
                if let Some(module) = &self.module {
                    let module_ptr = alloc::sync::Arc::as_ptr(module) as *mut Module;
                    *(*module_ptr)
                        .ast_types
                        .get_or_insert((*index_expr).index as *const AstExpr) =
                        (*self.builtin_types).stringType;
                }
                let key = (*self.dfg).get_refinement_key(index_expr as *const AstExpr);
                let value = (*constant_string).value;
                let bytes =
                    core::slice::from_raw_parts(value.data as *const u8, value.size as usize);
                let index: String = String::from(core::str::from_utf8(bytes).unwrap_or(""));
                return self.check_index_name(
                    scope,
                    key,
                    (*index_expr).expr,
                    &index,
                    (*index_expr).base.base.location,
                );
            }

            let obj = self.check_scope_ptr_ast_expr(scope, (*index_expr).expr).ty;
            let index_type = self.check_scope_ptr_ast_expr(scope, (*index_expr).index).ty;

            let result = (*self.arena).add_type(BlockedType::default());

            let key = (*self.dfg).get_refinement_key(index_expr as *const AstExpr);
            if !key.is_null() {
                // C++ default `prototype = true`.
                if let Some(ty) = self.lookup(
                    scope,
                    (*index_expr).base.base.location,
                    (*key).def as crate::type_aliases::def_id_def::DefId,
                    true,
                ) {
                    let refinement = self
                        .refinement_arena
                        .proposition_refinement_key_type_id(key, (*self.builtin_types).truthyType);
                    return Inference::inference_type_id_refinement_id(ty, refinement);
                }
                self.update_r_value_refinements_scope_ptr_def_id_type_id(
                    scope,
                    (*key).def as crate::type_aliases::def_id_def::DefId,
                    result,
                );
            }

            let c = self.add_constraint_scope_ptr_location_constraint_v(
                scope,
                (*(*index_expr).expr).base.location,
                ConstraintV::HasIndexer(HasIndexerConstraint {
                    result_type: result,
                    subject_type: obj,
                    index_type,
                }),
            );
            let blocked = get_mutable_type_id::<BlockedType>(result);
            (*blocked).set_owner(c as *const _);

            if !key.is_null() {
                let refinement = self
                    .refinement_arena
                    .proposition_refinement_key_type_id(key, (*self.builtin_types).truthyType);
                Inference::inference_type_id_refinement_id(result, refinement)
            } else {
                Inference::inference_type_id_refinement_id(result, core::ptr::null_mut())
            }
        }
    }
}
