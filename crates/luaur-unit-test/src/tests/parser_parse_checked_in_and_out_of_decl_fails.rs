#[cfg(test)]
#[test]
fn parser_parse_checked_in_and_out_of_decl_fails() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\n    local @checked = 3\n    @checked declare function abs(n: number): number\n",
    );
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;
    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.errors[0].get_location().begin.line, 1);
    assert_eq!(result.errors[1].get_location().begin.line, 1);
}
