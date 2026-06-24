//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1985:type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never

#[cfg(test)]
#[test]
fn type_infer_refinements_dataflow_analysis_can_tell_refinements_when_its_appropriate_to_refine_into_nil_or_never(
) {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(t: {string}, s: string)
            local v1 = t[5]
            local v2 = v1

            if typeof(v1) == "nil" then
                local foo = v1
            else
                local foo = v1
            end

            if typeof(v2) == "nil" then
                local foo = v2
            else
                local foo = v2
            end

            if typeof(s) == "nil" then
                local foo = s -- line 18
            else
                local foo = s -- line 20
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    assert_eq!(
        "nil",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(6, 28))
        )
    );
    assert_eq!(
        "string",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(8, 28))
        )
    );

    assert_eq!(
        "nil",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(12, 28))
        )
    );
    assert_eq!(
        "string",
        to_string_type_id(
            fixture
                .base
                .require_type_at_position_position(Position::new(14, 28))
        )
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "nil & string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(18, 28))
            )
        );
        assert_eq!(
            "string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(20, 28))
            )
        );
    } else {
        assert_eq!(
            "nil",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(18, 28))
            )
        );
        assert_eq!(
            "string",
            to_string_type_id(
                fixture
                    .base
                    .require_type_at_position_position(Position::new(20, 28))
            )
        );
    }
}
