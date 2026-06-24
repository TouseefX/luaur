#[cfg(test)]
#[test]
fn parser_lex_broken_unicode() {
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::lexeme::Lexeme;
    use luaur_ast::records::lexeme::Type;
    use luaur_ast::records::lexer::Lexer;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let test_input = [0xFFu8, 0xFE, 0xE2, 0x98, 0x83, 0xE2, 0x80, 0xA4];
    let mut alloc = Allocator::allocator();
    let mut names = AstNameTable::new(&mut alloc);
    let mut lexer = Lexer::new(
        test_input.as_ptr() as *const core::ffi::c_char,
        test_input.len(),
        &mut names,
        Position::default(),
    );

    let mut lexeme = unsafe { lexer.current().clone() };

    lexeme = unsafe { lexer.next().clone() };
    assert_eq!(lexeme.r#type, Type::BrokenUnicode);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0);
    assert_eq!(
        lexeme.location,
        Location::new(Position::new(0, 0), Position::new(0, 1))
    );

    lexeme = unsafe { lexer.next().clone() };
    assert_eq!(lexeme.r#type, Type::BrokenUnicode);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0);
    assert_eq!(
        lexeme.location,
        Location::new(Position::new(0, 1), Position::new(0, 2))
    );

    lexeme = unsafe { lexer.next().clone() };
    assert_eq!(lexeme.r#type, Type::BrokenUnicode);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0x2603);
    assert_eq!(
        lexeme.location,
        Location::new(Position::new(0, 2), Position::new(0, 5))
    );

    lexeme = unsafe { lexer.next().clone() };
    assert_eq!(lexeme.r#type, Type::BrokenUnicode);
    assert_eq!(unsafe { lexeme.data.codepoint }, 0x2024);
    assert_eq!(
        lexeme.location,
        Location::new(Position::new(0, 5), Position::new(0, 8))
    );

    lexeme = unsafe { lexer.next().clone() };
    assert_eq!(lexeme.r#type, Type::Eof);
}
