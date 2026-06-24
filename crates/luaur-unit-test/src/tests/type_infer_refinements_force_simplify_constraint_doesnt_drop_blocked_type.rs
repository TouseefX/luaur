//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2918:type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record SimplifyConstraint (Analysis/include/Luau/Constraint.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type

#[cfg(test)]
#[test]
fn type_infer_refinements_force_simplify_constraint_doesnt_drop_blocked_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function track(instance): boolean
            local isBasePart = instance:IsA("BasePart")
            local isCharacter = false
            if not isBasePart then
                isCharacter = instance:FindFirstChildOfClass("Humanoid") and instance:FindFirstChild("HumanoidRootPart")
            end
            -- A verison of `SimplifyConstraint` mucked up the fact that this
            -- is `boolean | and<unknown, unknown>`, and claimed it was only
            -- `boolean`.
            return isCharacter
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);
    type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
}
