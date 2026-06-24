//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1571:type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table`
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
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table

#[cfg(test)]
#[test]
fn type_infer_refinements_x_as_any_if_x_is_instance_elseif_x_is_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::refinement_extern_type_fixture::RefinementExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    // CLI-117136 - this code doesn't finish constraint solving and has blocked types in the output
    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = RefinementExternTypeFixture {
        base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict

        local function f(x)
            if typeof(x) == "Instance" and x:IsA("Folder") then
                local foo = x
            elseif typeof(x) == "table" then
                local foo = x
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Folder & Instance & {-  -}",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(5, 28))
            )
        );
        assert_eq!(
            "(~Folder | ~Instance) & {-  -} & never",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(7, 28))
            )
        );
    } else {
        assert_eq!(
            "Folder",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(5, 28))
            )
        );
        assert_eq!(
            "any",
            to_string_type_id(
                fixture
                    .base
                    .base
                    .require_type_at_position_position(Position::new(7, 28))
            )
        );
    }
}
