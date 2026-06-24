use crate::functions::follow_type::follow_type_id;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;

impl TypeChecker2 {
    /// C++ `TypeId TypeChecker2::lookupExpectedType(AstExpr* expr) const`.
    pub fn lookup_expected_type(&self, expr: *mut AstExpr) -> TypeId {
        let module = unsafe { &*self.module };

        if let Some(ty) = module.ast_expected_types.find(&(expr as *const AstExpr)) {
            unsafe { follow_type_id(*ty) }
        } else {
            unsafe { (*self.builtin_types).anyType }
        }
    }
}
