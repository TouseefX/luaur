#[cfg(test)]
#[test]
fn parser_unparenthesized_function_return_type_list() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("function foo(): string, number end"),
        &alloc::string::String::from("Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from("function foo(): (number) -> string, string"),
        &alloc::string::String::from("Expected a statement, got ','; did you forget to wrap the list of return types in parentheses?"),
        None,
    );

    // Will throw if the parse fails
    fixture.parse(
        r#"
        type Vector3MT = {
            __add: (Vector3MT, Vector3MT) -> Vector3MT,
            __mul: (Vector3MT, Vector3MT|number) -> Vector3MT
        }
    "#,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );
}
