#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_function_attributes() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut code = r#"
        @native
        function foo()
        end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    code = r#"
        @native
        local function foo()
        end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    code = r#"
        @checked local function foo()
        end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    code = r#"
        local foo = @native function() end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    code = r#"
        @native
        function foo:bar()
        end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    code = r#"
        @native   @checked
        function foo:bar()
        end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    {
        let _no_inline = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauNoInline, true);
        code = r#"
        @debugnoinline
        local function t() end
        "#;
        let result = pretty_print_string_view_parse_options_bool_bool(
            code,
            ParseOptions::default(),
            true,
            false,
        );
        assert_eq!(code, result.code);
    }
}
