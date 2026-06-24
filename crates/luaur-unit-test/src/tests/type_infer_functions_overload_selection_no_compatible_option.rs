//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3447:type_infer_functions_overload_selection_no_compatible_option`
//! Source: `tests/TypeInfer.functions.test.cpp`

#[cfg(test)]
#[test]
fn type_infer_functions_overload_selection_no_compatible_option() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local f: ((number) -> "one") & ((boolean) -> "two")
        local g = f("s" :: string)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
}
