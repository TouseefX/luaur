use crate::functions::follow_type::follow_type_id;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;

impl NonStrictTypeChecker {
    pub fn lookup_type(&mut self, expr: *mut AstExpr) -> TypeId {
        let module = unsafe { &*self.module };

        if let Some(ty) = module.ast_types.find(&(expr as *const AstExpr)) {
            let location = unsafe { (*expr).base.location };
            self.check_for_type_function_inhabitance(unsafe { follow_type_id(*ty) }, location)
        } else if let Some(tp) = module.ast_type_packs.find(&(expr as *const AstExpr)) {
            let location = unsafe { (*expr).base.location };
            let flattened = self.flatten_pack(*tp);
            self.check_for_type_function_inhabitance(flattened, location)
        } else {
            unsafe { (*self.builtin_types).anyType }
        }
    }
}
