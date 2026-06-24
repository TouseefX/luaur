//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:614:type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict

#[cfg(test)]
#[test]
fn type_infer_functions_duplicate_functions_with_different_signatures_not_allowed_in_nonstrict() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!nonstrict
        function foo(): number
            return 1
        end
        foo()

        function foo(n: number): number
            return 2
        end
        foo()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let tm = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("() -> number", to_string_type_id(tm.wanted_type));
    assert_eq!("(number) -> number", to_string_type_id(tm.given_type));
}
