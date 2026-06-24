#[cfg(test)]
#[test]
fn parser_basic_less_than_check_no_explicit_type_instantiaton() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
        r#"local a = b.c < d"#,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
    assert!(!_stat.is_null());
}
