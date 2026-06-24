#[cfg(test)]
#[test]
fn parser_recovery_error_limit_1() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::records::f_value::FValue;

    let mut fixture = Fixture::fixture_bool(false);
    let _guard = ScopedFastInt::new(&luaur_common::FInt::LuauParseErrorLimit, 1);

    let source = alloc::string::String::from("local a = ");
    let result = fixture.try_parse(
        &source,
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert_eq!(1, result.errors.len());
    assert_eq!(
        result.errors.first().unwrap().get_message(),
        &result.errors.first().unwrap().what().to_string()
    );
}
