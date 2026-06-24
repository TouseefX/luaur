//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_new_non_strict_should_suppress_unknown_require_errors() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::unknown_require::UnknownRequire;
    use luaur_ast::enums::mode::Mode;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    // Nonstrict mode: should suppress unknown require errors
    let result_nonstrict = fixture.base.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
require(script.NonExistent)
require("@self/NonExistent")
    "#,
        ),
        None,
    );

    assert_eq!(
        0,
        result_nonstrict.errors.len(),
        "{:?}",
        result_nonstrict.errors
    );

    // Strict mode: should report unknown require errors
    let result_strict = fixture.base.check_mode_string_optional_frontend_options(
        Mode::Strict,
        &String::from(
            r#"
require(script.NonExistent)
require("@self/NonExistent")
    "#,
        ),
        None,
    );

    assert_eq!(2, result_strict.errors.len(), "{:?}", result_strict.errors);
    assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[0]).is_some());
    assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[1]).is_some());
}
