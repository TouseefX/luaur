use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::symbol::Symbol;
use crate::records::type_checker::TypeChecker;
use crate::records::type_error::TypeError;
use crate::records::unknown_symbol::Context;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::type_aliases::scope_ptr_type_infer::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl TypeChecker {
    pub fn check_l_value_binding_scope_ptr_ast_expr_local(
        &mut self,
        scope: &ScopePtr,
        expr: &AstExprLocal,
    ) -> TypeId {
        let binding_opt = scope.lookup_symbol(Symbol::from_local(expr.local));
        if let Some(ty) = binding_opt {
            let ty = unsafe { follow_type_id(ty) };
            let ty_ptr = unsafe { get_type_id::<NeverType>(ty) };
            if !ty_ptr.is_null() {
                return self.unknown_type;
            }
            return ty;
        }

        let name_str =
            unsafe { core::ffi::CStr::from_ptr((*expr.local).name.value).to_string_lossy() };
        let error_data = TypeErrorData::UnknownSymbol(UnknownSymbol::new(
            name_str.to_string(),
            Context::Binding,
        ));
        let error =
            TypeError::type_error_location_type_error_data(expr.base.base.location, error_data);
        self.report_error_type_error(&error);

        self.error_recovery_type_scope_ptr(scope)
    }
}
