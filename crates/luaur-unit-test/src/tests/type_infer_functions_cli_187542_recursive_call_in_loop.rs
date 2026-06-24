#[cfg(test)]
#[test]
fn type_infer_functions_cli_187542_recursive_call_in_loop() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _crash_on_force = ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);
    let mut fixture = BuiltinsFixture::default();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function a(b)
            if true then return b end
            while false do
                b = a(b)
            end

            if true then return b end
        end
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert!(
        !result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<ConstraintSolvingIncompleteError>(error).is_some()),
        "expected no ConstraintSolvingIncompleteError, got {:?}",
        result.errors
    );
}
