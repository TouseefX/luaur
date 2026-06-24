//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_new_non_strict_should_suppress_dynamic_require_errors() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::unknown_require::UnknownRequire;
    use luaur_ast::enums::mode::Mode;
    use luaur_common::FFlag;

    let _sff_debug_luau_force_old_solver =
        ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    // Nonstrict mode should suppress dynamic require errors
    let result_nonstrict = fixture.base.check_mode_string_optional_frontend_options(
        Mode::Nonstrict,
        &String::from(
            r#"
function passThrough(module)
    require(module)
end
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

    // Strict mode should still warn about dynamic requires
    let result_strict = fixture.base.check_mode_string_optional_frontend_options(
        Mode::Strict,
        &String::from(
            r#"
function passThrough(module)
    require(module)
end
    "#,
        ),
        None,
    );

    assert_eq!(1, result_strict.errors.len(), "{:?}", result_strict.errors);
    assert!(type_error_data_ref::<UnknownRequire>(&result_strict.errors[0]).is_some());
}
