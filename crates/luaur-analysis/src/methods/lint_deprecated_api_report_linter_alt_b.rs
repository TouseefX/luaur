use crate::functions::emit_warning::emit_warning;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use core::ffi::c_char;
use luaur_ast::records::location::Location;
use luaur_config::enums::code::Code;

impl LintDeprecatedApi {
    pub fn report_location_c_char_c_char(
        &mut self,
        location: &Location,
        table_name: *const c_char,
        function_name: *const c_char,
    ) {
        let function_name = unsafe { core::ffi::CStr::from_ptr(function_name).to_string_lossy() };
        if !table_name.is_null() {
            let table_name = unsafe { core::ffi::CStr::from_ptr(table_name).to_string_lossy() };
            emit_warning(
                unsafe { &mut *self.context },
                Code::Code_DeprecatedApi,
                *location,
                format_args!("Member '{}.{}' is deprecated", table_name, function_name),
            );
        } else {
            emit_warning(
                unsafe { &mut *self.context },
                Code::Code_DeprecatedApi,
                *location,
                format_args!("Member '{}' is deprecated", function_name),
            );
        }
    }
}
