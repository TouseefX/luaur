//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:341:type_infer_tables_used_dot_instead_of_colon`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record FunctionRequiresSelf (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_used_dot_instead_of_colon

#[cfg(test)]
#[test]
fn type_infer_tables_used_dot_instead_of_colon() {
    use crate::functions::has_error::has_error;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::function_requires_self::FunctionRequiresSelf;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local T = {}
        T.x = 0
        function T:method()
            return self.x
        end
        local a = T.method()
    "#,
        ),
        None,
    );

    assert!(
        has_error::<FunctionRequiresSelf>(&result),
        "{:?}",
        result.errors
    );
}
