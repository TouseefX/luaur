#[cfg(test)]
#[test]
fn parser_end_extent_of_functions_unions_and_intersections() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from(
        "\n        type F = (string) -> string\n        type G = string | number | boolean\n        type H = string & number & boolean\n        print('hello')\n    ",
    );
    let result = fixture.try_parse(&source, &ParseOptions::parse_options());
    assert_eq!(4, unsafe { (*result.root).body.size });
    assert_eq!(luaur_ast::records::position::Position::new(1, 35), unsafe {
        (**(*result.root).body.data.add(0)).base.location.end
    });
    assert_eq!(luaur_ast::records::position::Position::new(2, 42), unsafe {
        (**(*result.root).body.data.add(1)).base.location.end
    });
    assert_eq!(luaur_ast::records::position::Position::new(3, 42), unsafe {
        (**(*result.root).body.data.add(2)).base.location.end
    });
}
