//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2882:type_infer_tables_cannot_call_tables`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotCallNonFunction (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_cannot_call_tables

#[cfg(test)]
#[test]
fn type_infer_tables_cannot_call_tables() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::cannot_call_non_function::CannotCallNonFunction;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture
        .check_string_optional_frontend_options(&String::from("local foo = {}    foo()"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<CannotCallNonFunction>(&result.errors[0])
        .expect("expected CannotCallNonFunction");
}
