//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1638:type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time

#[cfg(test)]
#[test]
fn type_infer_refinements_isa_type_refinement_must_be_known_ahead_of_time() {
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
        local function f(x): Instance
            if x:IsA("Folder") then
                local foo = x
            else
                local foo = x
            end

            return x
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "t1 where t1 = Instance & { read IsA: (t1, string) -> (unknown, ...unknown) }",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(3, 28))
            )
        );
        assert_eq!(
            "t1 where t1 = Instance & { read IsA: (t1, string) -> (unknown, ...unknown) }",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(5, 28))
            )
        );
    } else {
        assert_eq!(
            "Instance",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(3, 28))
            )
        );
        assert_eq!(
            "Instance",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(5, 28))
            )
        );
    }
}
