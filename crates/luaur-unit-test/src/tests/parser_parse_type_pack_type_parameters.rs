#[cfg(test)]
#[test]
fn parser_parse_type_pack_type_parameters() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let _stat = fixture.parse(
        r#"type Packed<T...> = () -> T...

type A<X...> = Packed<X...>
type B<X...> = Packed<...number>
type C<X...> = Packed<(number, X...)>
    "#,
        &ParseOptions::default(),
    );
    assert!(!_stat.is_null());
}
