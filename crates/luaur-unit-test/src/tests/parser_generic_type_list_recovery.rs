#[cfg(test)]
#[test]
fn parser_generic_type_list_recovery() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "local function foo<T..., U>(a: U, ...: T...): (U, ...T) return a, ... end\n\
         return foo(1, 2 -- to check for a second error after recovery",
    );

    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 2);
    assert_eq!(
        result.errors[0].get_message().as_str(),
        "Generic types come before generic type packs"
    );
}
