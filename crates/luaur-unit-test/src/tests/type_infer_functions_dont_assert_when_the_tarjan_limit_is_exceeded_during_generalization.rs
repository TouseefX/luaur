//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2172:type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record UnificationTooComplex (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization

#[cfg(test)]
#[test]
fn type_infer_functions_dont_assert_when_the_tarjan_limit_is_exceeded_during_generalization() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::records::unification_too_complex::UnificationTooComplex;
    use luaur_common::{FFlag, FInt};

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _tarjan_limit = ScopedFastInt::new(&FInt::LuauTarjanChildLimit, 1);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(t)
            t.x.y.z = 441
        end
    "#,
        ),
        None,
    );

    assert!(
        result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<UnificationTooComplex>(error).is_some()),
        "{:?}",
        result.errors
    );
}
