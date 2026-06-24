//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_unknown_globals_in_non_strict_1() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::enums::mode::Mode;

    let mut fixture = Fixture::default();

    let result = fixture.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
        foo = 5
        local wrong1 = foob

        local x = 12
        local wrong2 = x + foblm
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}
