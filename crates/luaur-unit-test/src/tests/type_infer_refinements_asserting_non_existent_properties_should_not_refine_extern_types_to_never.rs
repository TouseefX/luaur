//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1687:type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never`
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
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never

#[cfg(test)]
#[test]
fn type_infer_refinements_asserting_non_existent_properties_should_not_refine_extern_types_to_never()
{
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::refinement_extern_type_fixture::RefinementExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
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
        assert(weld.Part8)
        print(weld)
        assert(weld.Part8.Name == "RootPart")
        local part8 = assert(weld.Part8)
        local pos = part8.Position
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        to_string_type_error(&result.errors[0]),
        "Key 'Part8' not found in external type 'WeldConstraint'"
    );

    assert_eq!(
        "WeldConstraint",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(3, 15))
        )
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "any",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(6, 29))
            )
        );
    } else {
        assert_eq!(
            "*error-type*",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(6, 29))
            )
        );
    }
}
