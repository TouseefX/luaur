//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:1927:type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage

#[cfg(test)]
#[test]
fn type_infer_refinements_refine_a_param_that_got_resolved_during_constraint_solving_stage() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::refinement_extern_type_fixture::RefinementExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = RefinementExternTypeFixture {
        base: BuiltinsFixture::default(),
    };
    fixture.get_frontend();
    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Id<T> = T

        local function f(x: Id<Id<Part | Folder> | Id<string>>)
            if typeof(x) ~= "string" and x:IsA("Part") then
                local foo = x
            else
                local foo = x
            end
        end
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
    assert_eq!(
        "Folder | string",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position::new(7, 28))
        )
    );
}
