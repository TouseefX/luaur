use crate::records::lint_context::LintContext;
use alloc::string::String;
use luaur_ast::records::location::Location;
use luaur_common::functions::vformat::vformat;
use luaur_config::enums::code::Code;
use luaur_config::records::lint_warning::LintWarning;

pub fn emit_warning(
    context: &mut LintContext,
    code: Code,
    location: Location,
    args: core::fmt::Arguments<'_>,
) {
    if !context.warning_enabled(code) {
        return;
    }

    let message: String = vformat(args);
    let warning = LintWarning {
        code,
        location,
        text: message,
    };
    context.result.push(warning);
}
