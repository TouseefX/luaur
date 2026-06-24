#[cfg(test)]
#[test]
fn parser_allow_unicode_in_string() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    // C++ source is `local snowman = "☃"` (U+2603); the ported literal was
    // double-encoded UTF-8 mojibake. Use a \u escape (encoding-safe).
    let source = alloc::string::String::from("local snowman = \"\u{2603}\"");
    let options = ParseOptions::parse_options();
    let result = fixture.parse_ex(&source, &options);
    assert!(result.errors.is_empty());
}
