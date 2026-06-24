use crate::functions::emit_warning::emit_warning;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use core::ffi::c_char;
use luaur_ast::records::deprecated_info::DeprecatedInfo;
use luaur_ast::records::location::Location;
use luaur_config::enums::code::Code;

impl LintDeprecatedApi {
    pub fn report_location_c_char_ast_attr_deprecated_info(
        &mut self,
        location: &Location,
        function_name: *const c_char,
        info: &DeprecatedInfo,
    ) {
        let function_name = unsafe { core::ffi::CStr::from_ptr(function_name).to_string_lossy() };
        let use_part = info
            .use_suggestion()
            .map(|value| format!(", use '{}' instead", value))
            .unwrap_or_default();
        let reason_part = info
            .reason()
            .map(|value| format!(". {}", value))
            .unwrap_or_default();

        emit_warning(
            unsafe { &mut *self.context },
            Code::Code_DeprecatedApi,
            *location,
            format_args!(
                "Function '{}' is deprecated{}{}",
                function_name, use_part, reason_part
            ),
        );
    }
}
