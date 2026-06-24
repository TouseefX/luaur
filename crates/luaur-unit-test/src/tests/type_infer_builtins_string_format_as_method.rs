//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:842:type_infer_builtins_string_format_as_method`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_builtins_string_format_as_method

#[cfg(test)]
#[test]
fn type_infer_builtins_string_format_as_method() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture
        .check_string_optional_frontend_options(&String::from("local _ = ('%s'):format(5)"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!(fixture.get_builtins().stringType, mismatch.wanted_type);
    assert_eq!(fixture.get_builtins().numberType, mismatch.given_type);
}
