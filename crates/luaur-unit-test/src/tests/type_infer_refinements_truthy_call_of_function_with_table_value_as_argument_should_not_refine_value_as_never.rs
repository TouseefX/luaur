//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2589:type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never`
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
//!   - translates_to -> rust_item type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never

#[cfg(test)]
#[test]
fn type_infer_refinements_truthy_call_of_function_with_table_value_as_argument_should_not_refine_value_as_never(
) {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Item = {}

        local function predicate(value: Item): boolean
            return true
        end

        local function checkValue(value: Item)
            if predicate(value) then
                local _ = value
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Item",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(8, 27)))
    );
    assert_eq!(
        "Item",
        to_string_type_id(fixture.require_type_at_position_position(Position::new(9, 28)))
    );
}
