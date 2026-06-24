//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1627:type_infer_generics_apply_type_function_nested_generics_3`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_generics_apply_type_function_nested_generics_3

#[cfg(test)]
#[test]
fn type_infer_generics_apply_type_function_nested_generics_3() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local getReturnValue: <V>(cb: () -> V) -> V = nil :: any

        local y = getReturnValue(function() return nil :: any end)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
