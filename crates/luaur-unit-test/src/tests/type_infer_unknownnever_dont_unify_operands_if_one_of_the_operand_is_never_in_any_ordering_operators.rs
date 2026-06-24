//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.unknownnever.test.cpp:328:type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators`
//! Source: `tests/TypeInfer.unknownnever.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.unknownnever.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.unknownnever.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators

#[cfg(test)]
#[test]
fn type_infer_unknownnever_dont_unify_operands_if_one_of_the_operand_is_never_in_any_ordering_operators(
) {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function ord(x: nil, y)
            return x ~= nil and x > y
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(nil, nil & ~nil) -> boolean",
            to_string_type_id(fixture.require_type_string(&String::from("ord")))
        );
    } else {
        assert_eq!(
            "<a>(nil, a) -> boolean",
            to_string_type_id(fixture.require_type_string(&String::from("ord")))
        );
    }
}
