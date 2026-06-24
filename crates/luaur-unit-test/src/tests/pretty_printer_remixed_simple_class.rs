#[cfg(test)]
#[test]
fn pretty_printer_remixed_simple_class() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    let _fflag = ScopedFastFlag::new(&luaur_common::FFlag::DebugLuauUserDefinedClasses, true);

    let code = r#"
class Point
    function length(self)
        return 100
    end
    public x
    function new(): Point
        return Point { x = 0, y = 0 }
    end
    public y
end
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);
}
