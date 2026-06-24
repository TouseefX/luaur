#[cfg(test)]
#[test]
fn parser_vertical_space() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    // C++ "a()\vb()" — \v is a vertical tab (U+000B). Rust has no \v escape and a
    // raw string would embed a literal backslash-v, so use an explicit \u escape.
    let source = alloc::string::String::from("a()\u{0B}b()");
    let options = ParseOptions::parse_options();
    let result = fixture.parse_ex(&source, &options);
    assert!(result.errors.is_empty());
}
