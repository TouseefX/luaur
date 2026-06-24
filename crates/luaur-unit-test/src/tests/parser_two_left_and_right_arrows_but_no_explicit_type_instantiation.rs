#[cfg(test)]
#[test]
fn parser_two_left_and_right_arrows_but_no_explicit_type_instantiation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(r#"type A = C<B<<T>() -> T>>"#, &ParseOptions::default());
    assert!(!_stat.is_null());
}
