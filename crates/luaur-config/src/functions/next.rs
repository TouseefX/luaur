use luaur_ast::records::lexeme::Type;
use luaur_ast::records::lexer::Lexer;

pub(crate) fn next(lexer: &mut Lexer) {
    lexer.next();

    // skip C-style comments as Lexer only understands Lua-style comments atm
    while lexer.current().r#type == Type::FloorDiv {
        lexer.nextline();
    }
}
