//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:248:type_infer_extern_types_warn_when_prop_almost_matches`
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
//!   - type_ref -> record UnknownPropButFoundLikeProp (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_extern_types_warn_when_prop_almost_matches

#[cfg(test)]
#[test]
fn type_infer_extern_types_warn_when_prop_almost_matches() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::unknown_prop_but_found_like_prop::UnknownPropButFoundLikeProp;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        Vector2.new(0, 0)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = unsafe { get_type_error::<UnknownPropButFoundLikeProp>(&result.errors[0]).as_ref() }
        .expect("expected UnknownPropButFoundLikeProp");
    assert_eq!(1, err.candidates().len());
    assert!(err.candidates().contains("New"));
}
