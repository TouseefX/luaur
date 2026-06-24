use crate::records::symbol::Symbol;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::unknown_symbol::{Context, UnknownSymbol};
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl TypeChecker2 {
    pub fn visit_ast_expr_global(&mut self, expr: *mut AstExprGlobal) {
        unsafe {
            let scope = *self
                .stack
                .last()
                .expect("TypeChecker2 stack should not be empty");
            let name = (*expr).name;
            let name_string = core::ffi::CStr::from_ptr(name.value)
                .to_string_lossy()
                .into_owned();

            if (*scope).lookup_symbol(Symbol::from_global(name)).is_none() {
                self.report_error_type_error_data_location(
                    UnknownSymbol::new(name_string, Context::Binding).into(),
                    &(*expr).base.base.location,
                );
            } else if (*scope).should_warn_global(name_string.clone())
                && !self.warned_globals.contains(&name_string)
            {
                self.report_error_type_error_data_location(
                    UnknownSymbol::new(name_string.clone(), Context::Binding).into(),
                    &(*expr).base.base.location,
                );
                self.warned_globals.insert(name_string);
            }
        }
    }
}
