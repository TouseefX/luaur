use crate::functions::get_type_alt_j::get_type_id;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use crate::records::table_type::TableType;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::location::Location;

impl LintDeprecatedApi {
    pub fn check_location_ast_name_ast_name(
        &mut self,
        location: &Location,
        global: AstName,
        index: AstName,
    ) {
        let Some(global_value) = (unsafe { &*self.context }).builtin_globals.find(&global) else {
            return;
        };

        let table = unsafe { get_type_id::<TableType>(global_value.r#type) };
        if table.is_null() {
            return;
        }

        let index_name = unsafe {
            core::ffi::CStr::from_ptr(index.value)
                .to_string_lossy()
                .into_owned()
        };

        if let Some(prop) = unsafe { &*table }.props.get(&index_name) {
            if prop.deprecated {
                self.report_location_property_c_char_c_char(
                    location,
                    prop,
                    global.value,
                    index.value,
                );
            }
        }
    }
}
