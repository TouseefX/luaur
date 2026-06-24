#[cfg(test)]
#[test]
fn parser_parse_checked_outside_decl_fails() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let source = alloc::string::String::from(
        "local @checked = 3
",
    );
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;

    let result = fixture.try_parse(&source, &opts);

    assert!(result.errors.len() > 0);
    let _ts = result.errors[1].get_message();
}
