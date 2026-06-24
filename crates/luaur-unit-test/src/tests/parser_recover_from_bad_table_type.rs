#[cfg(test)]
#[test]
fn parser_recover_from_bad_table_type() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\n        declare class Widget\n            state: {string: function(string, Widget)}\n        end\n    ",
    );
    let mut opts = ParseOptions::parse_options();
    opts.allow_declaration_syntax = true;
    let result = fix.try_parse(&code, &opts);
    assert_eq!(result.errors.len(), 2);
}
