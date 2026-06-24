//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2076:type_infer_fuzz_generalize_one_remove_type_assert_2`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record ConstraintSolvingIncompleteError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_fuzz_generalize_one_remove_type_assert_2

#[cfg(test)]
#[test]
fn type_infer_fuzz_generalize_one_remove_type_assert_2() {
    use crate::functions::has_error::has_error;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::constraint_solving_incomplete_error::ConstraintSolvingIncompleteError;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local _ = {n0 = _.n0}, -_, _
        _ += _.n0
        _ /= _[_]
        while _.n110 do
            while _._ do
                while _ do
                    while _ do
                        _ = _
                    end
                end
                while _[_] do
                    function _()
                    end
                end
            end
            while ... do
            end
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert!(
        !has_error::<ConstraintSolvingIncompleteError>(&result),
        "{:?}",
        result.errors
    );
}
