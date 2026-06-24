//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_unknown_globals_in_one_sided_conditionals() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::unknown_symbol::Context;
    use luaur_analysis::records::unknown_symbol::UnknownSymbol;
    use luaur_ast::enums::mode::Mode;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
        local function foo(cond) : ()
            if cond then
                bar()
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
        type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("bar", err.name());
    assert_eq!(Context::Binding, err.context());
}
