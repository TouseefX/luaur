#[cfg(test)]
#[test]
fn pretty_printer_export() {
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag;

    let _export_value = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);

    let mut code = r#"
export                      local version = "1.0.0"
export           const tabbed = ...
export const TAU = math.pi * 2
export local settings: Settings = getSettings()
export local a, b, c = 1, 2, 3
export local d
    "#;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    code = r#"
export function add(a: number, b: number): number
    return a + b
end

export function greet(name: string): string
    return "Hello, " .. name
end

export function noop()
end

export        function tabbed(): number
    return 1
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
export function foo()
end

@native
export                 function tabbed_attribute()
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
export local f, g

function f()
    return g()
end

function g()
    return 42
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
export type Config = {
    debug: boolean,
    timeout: number,
}

export local currentConfig: Config

export function createConfig(debug: boolean, timeout: number): Config
    return {
        debug = debug,
        timeout = timeout,
    }
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
