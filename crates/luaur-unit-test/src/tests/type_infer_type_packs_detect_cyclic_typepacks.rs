//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1015:type_infer_type_packs_detect_cyclic_typepacks`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_type_packs_detect_cyclic_typepacks

#[cfg(test)]
#[test]
fn type_infer_type_packs_detect_cyclic_typepacks() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type ( ... ) ( ) ;
        ( ... ) ( - - ... ) ( - ... )
        type = ( ... ) ;
        ( ... ) (  ) ( ... ) ;
        ( ... ) ""
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
