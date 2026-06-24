//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1666:type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never

#[cfg(test)]
#[test]
fn type_infer_refinements_asserting_optional_properties_should_not_refine_extern_types_to_never() {
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
        local weld: WeldConstraint = nil :: any
        assert(weld.Part1)
        print(weld) -- hover type incorrectly becomes `never`
        assert(weld.Part1.Name == "RootPart")
        local part1 = assert(weld.Part1)
        local pos = part1.Position
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() && FFlag::LuauExternTypesNormalizeWithShapes.get() {
        assert_eq!(
            "WeldConstraint & { read Part1: ~(false?) }",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(3, 15))
            )
        );
    } else {
        assert_eq!(
            "WeldConstraint",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(3, 15))
            )
        );
    }
    assert_eq!(
        "Vector3",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(6, 29))
        )
    );
}
