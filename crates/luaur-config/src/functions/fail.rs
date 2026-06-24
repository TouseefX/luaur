use crate::type_aliases::error::Error;
use luaur_ast::records::lexer::Lexer;
use luaur_common::functions::format::format;

pub(crate) fn fail(lexer: &Lexer, message: &str) -> Error {
    let cur = lexer.current();
    Some(format(format_args!(
        "Expected {} at line {}, got {} instead",
        message,
        cur.location.begin.line + 1,
        cur.to_string()
    )))
}
