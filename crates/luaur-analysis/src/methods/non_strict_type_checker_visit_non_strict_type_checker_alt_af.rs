use crate::enums::value_context::ValueContext;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::symbol::Symbol;
use crate::records::unknown_symbol::UnknownSymbol;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl NonStrictTypeChecker {
    pub fn visit_ast_expr_global_value_context(
        &mut self,
        global: *mut AstExprGlobal,
        context: ValueContext,
    ) -> NonStrictContext {
        // We don't file unknown symbols for LValues.
        if context == ValueContext::LValue {
            return NonStrictContext::non_strict_context();
        }

        let Some(scope) = self.stack.last().copied() else {
            return NonStrictContext::non_strict_context();
        };

        let sym = unsafe { Symbol::from_global((*global).name) };
        if unsafe { (*scope).lookup_symbol(sym).is_none() } {
            let name_str =
                unsafe { core::ffi::CStr::from_ptr((*global).name.value).to_string_lossy() };
            let error_data = TypeErrorData::UnknownSymbol(UnknownSymbol::new(
                name_str.to_string(),
                crate::records::unknown_symbol::UnknownSymbol_Context::Binding,
            ));
            unsafe { self.report_error(error_data, &(*global).base.base.location) };
        }

        NonStrictContext::non_strict_context()
    }
}
