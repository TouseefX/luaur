//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1946:type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2

#[cfg(test)]
#[test]
fn type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage_2() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::refinement_extern_type_fixture::RefinementExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = RefinementExternTypeFixture {
        base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function hof(f: (Instance) -> ()) end

        hof(function(inst)
            if inst:IsA("Part") then
                local foo = inst
            else
                local foo = inst
            end
        end)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Part",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(5, 28))
        )
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Instance & ~Part",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(7, 28))
            )
        );
    } else {
        assert_eq!(
            "Instance",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(7, 28))
            )
        );
    }
}
