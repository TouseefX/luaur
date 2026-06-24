//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_optional_length_error() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {number}
        function f(a: A?)
            local b = #a
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Operator '#' could not be applied to operand of type A?; there is no corresponding overload for __len",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Value of type 'A?' could be nil",
        to_string_type_error(&result.errors[1])
    );
}
