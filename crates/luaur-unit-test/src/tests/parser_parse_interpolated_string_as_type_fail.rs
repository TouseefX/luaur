#[cfg(test)]
#[test]
fn parser_parse_interpolated_string_as_type_fail() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let parse_options = ParseOptions::default();

    let source = alloc::string::String::from(
        "\n            local a: `what` = `???`\n            local b: `what {\"the\"}` = `???`\n            local c: `what {\"the\"} heck` = `???`\n        ",
    );

    // C++ catches ParseErrors and checks getErrors().size() == 3, each with the same
    // message. The port wrapped fixture.parse (which panics with a "ParseErrors: ..."
    // prefix) and only checked one message. Use try_parse and inspect all 3 errors.
    let result = fixture.try_parse(&source, &parse_options);

    assert_eq!(3, result.errors.len());
    for error in result.errors.iter() {
        assert_eq!(
            "Interpolated string literals cannot be used as types",
            error.get_message().as_str()
        );
    }
}
