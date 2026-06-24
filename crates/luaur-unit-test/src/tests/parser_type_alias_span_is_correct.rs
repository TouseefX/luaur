#[cfg(test)]
#[test]
fn parser_type_alias_span_is_correct() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "\n        type Packed1<T...> = (T...) -> (T...)\n        type Packed2<T...> = (Packed1<T...>, T...) -> (Packed1<T...>, T...)\n    ",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let root = unsafe { &*block };
    assert_eq!(root.body.size, 2);

    let t1 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
            *root.body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!t1.is_null());
    let t1 = unsafe { &*t1 };
    assert_eq!(
        t1.base.base.location,
        Location::new(Position::new(1, 8), Position::new(1, 45))
    );

    let t2 = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatTypeAlias>(
            *root.body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!t2.is_null());
    let t2 = unsafe { &*t2 };
    assert_eq!(
        t2.base.base.location,
        Location::new(Position::new(2, 8), Position::new(2, 75))
    );
}
