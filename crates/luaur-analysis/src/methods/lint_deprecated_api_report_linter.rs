use crate::functions::emit_warning::emit_warning;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use crate::records::property_type::Property;
use core::ffi::c_char;
use luaur_ast::records::location::Location;
use luaur_config::enums::code::Code;

impl LintDeprecatedApi {
    pub fn report_location_property_c_char_c_char(
        &mut self,
        location: &Location,
        prop: &Property,
        container: *const c_char,
        field: *const c_char,
    ) {
        let suggestion = if prop.deprecated_suggestion.is_empty() {
            ""
        } else {
            prop.deprecated_suggestion.as_str()
        };
        let field = unsafe { core::ffi::CStr::from_ptr(field).to_string_lossy() };

        if !container.is_null() {
            let container = unsafe { core::ffi::CStr::from_ptr(container).to_string_lossy() };
            if suggestion.is_empty() {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_DeprecatedApi,
                    *location,
                    format_args!("Member '{}.{}' is deprecated", container, field),
                );
            } else {
                emit_warning(
                    unsafe { &mut *self.context },
                    Code::Code_DeprecatedApi,
                    *location,
                    format_args!(
                        "Member '{}.{}' is deprecated, use '{}' instead",
                        container, field, suggestion
                    ),
                );
            }
        } else if suggestion.is_empty() {
            emit_warning(
                unsafe { &mut *self.context },
                Code::Code_DeprecatedApi,
                *location,
                format_args!("Member '{}' is deprecated", field),
            );
        } else {
            emit_warning(
                unsafe { &mut *self.context },
                Code::Code_DeprecatedApi,
                *location,
                format_args!(
                    "Member '{}' is deprecated, use '{}' instead",
                    field, suggestion
                ),
            );
        }
    }
}
