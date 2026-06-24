#[cfg(test)]
#[test]
fn parser_error_message_for_using_function_as_type_annotation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from("type Foo = function");
    let result = fix.try_parse(&code, &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        result.errors[0].get_message().as_str(),
        "Using 'function' as a type annotation is not supported, consider replacing with a function type annotation e.g. '(...any) -> ...any'"
    );
}
