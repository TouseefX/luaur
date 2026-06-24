#[cfg(test)]
#[test]
fn parser_parsing_type_suffix_for_return_type_with_variadic() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::DFFlag;

    // C++ sets the DFFlag (which gates the parser's telemetry side-effect) and then
    // checks the telemetry GLOBAL, not the flag itself.
    let _sff = DFFlag::DebugLuauReportReturnTypeVariadicWithTypeSuffix.set(true);
    unsafe {
        luaur_ast::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX = false;
    }

    let mut fix = Fixture::default();
    let code = r"function foo(): (string, ...number) | boolean
end";

    let result = fix.try_parse(&code.to_string(), &ParseOptions::parse_options());

    // TODO(CLI-140667): this should produce a ParseError in future when we fix the invalid syntax
    assert_eq!(result.errors.len(), 0);
    assert!(unsafe { luaur_ast::LUAU_TELEMETRY_PARSED_RETURN_TYPE_VARIADIC_WITH_TYPE_SUFFIX });
}
