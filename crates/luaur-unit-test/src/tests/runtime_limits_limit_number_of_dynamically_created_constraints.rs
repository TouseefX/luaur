//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_limit_number_of_dynamically_created_constraints() {
    use crate::functions::has_error::has_error;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::code_too_complex::CodeTooComplex;
    use luaur_common::{FFlag, FInt};

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let src = String::from(
        r#"
        type Array<T> = {T}

        type Hello = Array<Array<Array<Array<Array<Array<Array<Array<Array<Array<number>>>>>>>>>>
    "#,
    );

    {
        let _limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 5);
        let mut fixture = Fixture::fixture_bool(false);
        let result = fixture.check_string_optional_frontend_options(&src, None);
        let dynamic_constraints_created = fixture
            .frontend
            .as_ref()
            .expect("frontend should be initialized")
            .stats
            .dynamic_constraints_created;
        assert!(
            dynamic_constraints_created > 3,
            "dynamic constraints created: {dynamic_constraints_created}, errors: {:?}",
            result.errors
        );
        assert!(has_error::<CodeTooComplex>(&result), "{:?}", result.errors);
    }

    {
        let _limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 1000);
        let mut fixture = Fixture::fixture_bool(false);
        let result = fixture.check_string_optional_frontend_options(&src, None);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
    }

    {
        let _limit = ScopedFastInt::new(&FInt::LuauSolverConstraintLimit, 0);
        let mut fixture = Fixture::fixture_bool(false);
        let result = fixture.check_string_optional_frontend_options(&src, None);
        assert!(result.errors.is_empty(), "{:?}", result.errors);
    }
}
