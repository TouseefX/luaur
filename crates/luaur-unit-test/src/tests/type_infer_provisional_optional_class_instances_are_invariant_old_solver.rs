//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:982:type_infer_provisional_optional_class_instances_are_invariant_old_solver`
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
//!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method Lexer::current (Ast/include/Luau/Lexer.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_provisional_optional_class_instances_are_invariant_old_solver

#[cfg(test)]
#[test]
fn type_infer_provisional_optional_class_instances_are_invariant_old_solver() {
    use crate::functions::create_some_extern_types::create_some_extern_types;
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    create_some_extern_types(fixture.get_frontend());
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(ref: {current: Parent?})
        end

        function bar(ref: {current: Child?})
            foo(ref)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
