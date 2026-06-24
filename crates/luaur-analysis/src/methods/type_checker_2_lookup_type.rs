//! Faithful port of `TypeChecker2::lookupType` (TypeChecker2.cpp:522-536).
use crate::functions::follow_type::follow_type_id;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;

impl TypeChecker2 {
    pub fn lookup_type(&mut self, expr: *mut AstExpr) -> TypeId {
        // If a type isn't in the type graph, it probably means that a recursion limit was exceeded.
        // We'll just return anyType in these cases.  Typechecking against any is very fast and this
        // allows us not to think about this very much in the actual typechecking logic.
        let location = unsafe { (*expr).base.location };

        let ty = unsafe {
            (*self.module)
                .ast_types
                .find(&(expr as *const AstExpr))
                .copied()
        };
        if let Some(ty) = ty {
            return self
                .check_for_type_function_inhabitance(unsafe { follow_type_id(ty) }, location);
        }

        let tp = unsafe {
            (*self.module)
                .ast_type_packs
                .find(&(expr as *const AstExpr))
                .copied()
        };
        if let Some(tp) = tp {
            let flattened = self.flatten_pack(tp);
            return self.check_for_type_function_inhabitance(flattened, location);
        }

        unsafe { (*self.builtin_types).anyType }
    }
}
