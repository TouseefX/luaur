//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:3105:type_infer_refinements_type_function_reduction_with_union_type_application`
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
//!   - translates_to -> rust_item type_infer_refinements_type_function_reduction_with_union_type_application

#[cfg(test)]
#[test]
fn type_infer_refinements_type_function_reduction_with_union_type_application() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _assert_on_forced_constraint =
        ScopedFastFlag::new(&FFlag::DebugLuauAssertOnForcedConstraint, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local lastTick = 0
        local jumpAnimTime = 0
        local toolAnimTime = 0

        function move(time, tool, animStringValueObject)
            local deltaTime = time - lastTick
            lastTick = time

            if jumpAnimTime > 0 then
                jumpAnimTime = jumpAnimTime - deltaTime
            end

            if animStringValueObject then
                toolAnimTime = time + .3
            end

            if time > toolAnimTime then
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
