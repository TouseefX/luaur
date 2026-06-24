//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:427:type_infer_extern_types_optional_class_field_access_error`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_extern_types_optional_class_field_access_error

#[cfg(test)]
#[test]
fn type_infer_extern_types_optional_class_field_access_error() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local b: Vector2? = nil
local a = b.X + b.Z

b.X = 2 -- real Vector2.X is also read-only
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Value of type 'Vector2?' could be nil",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Value of type 'Vector2?' could be nil",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        "Key 'Z' not found in external type 'Vector2'",
        to_string_type_error(&result.errors[2])
    );
    assert_eq!(
        "Value of type 'Vector2?' could be nil",
        to_string_type_error(&result.errors[3])
    );
}
