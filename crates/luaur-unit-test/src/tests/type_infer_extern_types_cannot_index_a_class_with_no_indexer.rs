//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:846:type_infer_extern_types_cannot_index_a_class_with_no_indexer`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record DynamicPropertyLookupOnExternTypesUnsafe (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_extern_types_cannot_index_a_class_with_no_indexer

#[cfg(test)]
#[test]
fn type_infer_extern_types_cannot_index_a_class_with_no_indexer() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::dynamic_property_lookup_on_extern_types_unsafe::DynamicPropertyLookupOnExternTypesUnsafe;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = BaseClass.New()

        local c = a[1]
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe {
        get_type_error::<DynamicPropertyLookupOnExternTypesUnsafe>(&result.errors[0]).as_ref()
    };
    assert!(
        err.is_some(),
        "expected DynamicPropertyLookupOnExternTypesUnsafe but got {:?}",
        result.errors[0]
    );
    assert_eq!(
        unsafe { (*fixture.base.base.builtin_types).errorType },
        fixture.base.base.require_type_string(&String::from("c"))
    );
}
