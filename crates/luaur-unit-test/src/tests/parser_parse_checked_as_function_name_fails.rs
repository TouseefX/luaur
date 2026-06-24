#[cfg(test)]
#[test]
fn parser_parse_checked_as_function_name_fails() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = r"(
    @checked function(x: number) : number
    end
)";
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;

    let result = fix.try_parse(&alloc::string::String::from(code), &opts);
    assert!(result.errors.len() > 0);
}
