use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::inference::Inference;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::location::Location;

impl ConstraintGenerator {
    pub fn check_scope_ptr_ast_expr_binary_optional_type_id(
        &mut self,
        scope: &ScopePtr,
        binary: *mut AstExprBinary,
        expected_type: Option<TypeId>,
    ) -> TypeId {
        let location = unsafe { &(*binary).base.base.location };

        let inference = self.check_ast_expr_binary(
            scope,
            *location,
            unsafe { (*binary).op },
            unsafe { (*binary).left },
            unsafe { (*binary).right },
            expected_type,
        );

        inference.ty
    }
}
