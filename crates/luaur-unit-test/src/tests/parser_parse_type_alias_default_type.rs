#[cfg(test)]
#[test]
fn parser_parse_type_alias_default_type() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse(
        r#"type A<T = string> = {}
type B<T... = ...number> = {}
type C<T..., U... = T...> = {}
type D<T..., U... = ()> = {}
type E<T... = (), U... = ()> = {}
type F<T... = (string), U... = ()> = (T...) -> U...
type G<T... = ...number, U... = (string, number, boolean)> = (U...) -> T...
    "#,
        &ParseOptions::default(),
    );
    assert!(!stat.is_null());
}
