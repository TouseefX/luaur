//! Source: `Analysis/src/ConstraintGenerator.cpp:3281-3285` (hand-ported)
//! C++ `Inference ConstraintGenerator::check(const ScopePtr& scope, AstExprIndexName* indexName)`.
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use alloc::string::String;
use core::ffi::CStr;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_index_name(
        &mut self,
        scope: &ScopePtr,
        index_name: *mut AstExprIndexName,
    ) -> Inference {
        unsafe {
            let key = (*self.dfg).get_refinement_key(index_name as *const AstExpr);
            let index: String = CStr::from_ptr((*index_name).index.value)
                .to_string_lossy()
                .into_owned();
            self.check_index_name(
                scope,
                key,
                (*index_name).expr,
                &index,
                (*index_name).index_location,
            )
        }
    }
}
