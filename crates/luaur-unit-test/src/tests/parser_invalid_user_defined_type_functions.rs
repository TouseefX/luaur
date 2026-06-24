#[cfg(test)]
#[test]
fn parser_invalid_user_defined_type_functions() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();
    fixture.match_parse_error(
        &alloc::string::String::from("local foo = 1; type function bar() print(foo) end"),
        &alloc::string::String::from("Type function cannot reference outer local 'foo'"),
        None,
    );
    fixture.match_parse_error(
        &alloc::string::String::from(
            "type function foo() local v1 = 1; type function bar() print(v1) end end",
        ),
        &alloc::string::String::from("Type function cannot reference outer local 'v1'"),
        None,
    );
}
