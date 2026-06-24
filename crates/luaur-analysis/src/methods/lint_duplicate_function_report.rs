use crate::records::lint_duplicate_function::LintDuplicateFunction;
use luaur_ast::records::location::Location;
use luaur_config::enums::code::Code;

impl LintDuplicateFunction {
    pub fn report_location_c_char_location(
        &mut self,
        name: &str,
        location: Location,
        other_location: Location,
    ) {
        crate::functions::emit_warning::emit_warning(
            unsafe { &mut *self.context },
            Code::Code_DuplicateFunction,
            location,
            format_args!(
                "Duplicate function definition: '{}' also defined on line {}",
                name,
                other_location.begin.line + 1
            ),
        );
    }
}
