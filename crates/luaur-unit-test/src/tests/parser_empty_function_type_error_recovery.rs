#[cfg(test)]
#[test]
fn parser_empty_function_type_error_recovery() {
    use crate::records::fixture::Fixture;

    let mut fixture = Fixture::default();

    // C++ uses `parse(...)` (which throws ParseErrors) and checks the first error
    // message. The port called the unrelated `parseType` frontend stub and ignored
    // the result entirely. Use `match_parse_error`, which parses and asserts the
    // first error message.

    // Case 1: empty `()` in a union — special-cased error message.
    fixture.match_parse_error(
        &alloc::string::String::from(
            "\ntype Fn = (\n    any,\n    string | number | ()\n) -> any\n",
        ),
        &alloc::string::String::from(
            "Expected '->' after '()' when parsing function type; did you mean 'nil'?",
        ),
        None,
    );

    // Case 2: arguments present, no special case.
    fixture.match_parse_error(
        &alloc::string::String::from("type Fn = (any, string | number | (number, number)) -> any"),
        &alloc::string::String::from("Expected '->' when parsing function type, got ')'"),
        None,
    );

    // Case 3: generic arguments present, no special case.
    fixture.match_parse_error(
        &alloc::string::String::from("type Fn = (any, string | number | <a>()) -> any"),
        &alloc::string::String::from("Expected '->' when parsing function type, got ')'"),
        None,
    );

    // Case 4: variadic generic arguments present, no special case.
    fixture.match_parse_error(
        &alloc::string::String::from("type Fn = (any, string | number | <a...>()) -> any"),
        &alloc::string::String::from("Expected '->' when parsing function type, got ')'"),
        None,
    );
}
