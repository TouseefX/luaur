#[cfg(test)]
#[test]
fn parser_recover_unexpected_type_pack() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    let source = alloc::string::String::from(
        "type X<T...> = { a: T..., b: number }\n\
         type Y<T> = { a: T..., b: number }\n\
         type Z<T> = { a: string | T..., b: number }",
    );
    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );
    assert_eq!(3, result.errors.len());
}
