//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:206:ast_query_last_argument_function_call_type`
//! Source: `tests/AstQuery.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstQuery.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file tests/AstQueryDsl.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/AstQuery.test.cpp
//! - outgoing:
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> method Fixture::findExpectedTypeAtPosition (tests/Fixture.cpp)
//!   - translates_to -> rust_item ast_query_last_argument_function_call_type

#[cfg(test)]
#[test]
fn ast_query_last_argument_function_call_type() {
    use crate::tests::ast_query_support::*;

    let mut fixture = Fixture::default();
    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local function foo() return 2 end
local function bar(a: number) return -a end
bar(foo())
    "#,
        ),
        None,
    );

    let oty = fixture.find_type_at_position_position(Position { line: 3, column: 7 });
    assert!(oty.is_some());
    assert_eq!("number", to_string_type_id(oty.unwrap()));

    let expected_oty = fixture.find_expected_type_at_position(Position { line: 3, column: 7 });
    assert!(expected_oty.is_some());
    assert_eq!("number", to_string_type_id(expected_oty.unwrap()));
}
