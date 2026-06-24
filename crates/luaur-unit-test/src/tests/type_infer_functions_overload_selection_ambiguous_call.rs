//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:3409:type_infer_functions_overload_selection_ambiguous_call`
//! Source: `tests/TypeInfer.functions.test.cpp`

#[cfg(test)]
#[test]
fn type_infer_functions_overload_selection_ambiguous_call() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::ambiguous_function_call::AmbiguousFunctionCall;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local f: ((number | string) -> "one") & ((number | boolean) -> "two")
        local g = f(42)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<AmbiguousFunctionCall>(&result.errors[0])
        .expect("expected AmbiguousFunctionCall");
    assert_eq!("number", to_string_type_pack_id(err.arguments()));
    assert_eq!(
        "((boolean | number) -> \"two\") & ((number | string) -> \"one\")",
        to_string_type_id(err.function())
    );
    assert_eq!(
        "*error-type*",
        to_string_type_id(fixture.require_type_string(&String::from("g")))
    );
}
