//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:215:type_infer_provisional_while_body_are_also_refined`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Node (Analysis/src/TopoSortStatements.cpp)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
//!   - translates_to -> rust_item type_infer_provisional_while_body_are_also_refined

#[cfg(test)]
#[test]
fn type_infer_provisional_while_body_are_also_refined() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Node<T> = { value: T, child: Node<T>? }

        local function visitor<T>(node: Node<T>, f: (T) -> ())
            local current = node

            while current do
                f(current.value)
                current = current.child -- TODO: Can't work just yet. It thinks 'current' can never be nil. :(
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be 'Node<T>', but got 'Node<T>?'",
        to_string_type_error(&result.errors[0])
    );
}
