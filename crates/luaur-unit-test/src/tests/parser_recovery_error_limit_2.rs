#[cfg(test)]
#[test]
fn parser_recovery_error_limit_2() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_ast::records::parse_errors::ParseErrors;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FInt;

    let _sfi = ScopedFastInt::new(&FInt::LuauParseErrorLimit, 2);

    let mut fix = Fixture::default();

    // C++: parse(...) throws ParseErrors; FAIL if it does not. Fixture::parse
    // panics with the ParseErrors payload (see fixture_parse.rs).
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        fix.parse("escape escape escape", &ParseOptions::parse_options());
    }));
    assert!(result.is_err(), "Expected ParseErrors to be thrown");

    let err = result.unwrap_err();
    let errors = err
        .downcast_ref::<ParseErrors>()
        .expect("Expected ParseErrors");
    assert_eq!(errors.get_errors().len(), 3);
    assert_eq!(errors.what(), "3 parse errors");
    assert_eq!(
        errors.get_errors().last().unwrap().get_message().as_str(),
        "Reached error limit (2)"
    );
}
