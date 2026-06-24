use alloc::string::String;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_config::records::lint_warning::LintWarning;

pub fn check_deprecated_warning(warning: &LintWarning, begin: Position, end: Position, msg: &str) {
    assert_eq!(warning.code, LintWarning::Code_DeprecatedApi);
    assert_eq!(warning.location, Location::new(begin, end));
    assert_eq!(warning.text, msg);
}
