//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:764:type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param

#[cfg(test)]
#[test]
fn type_infer_provisional_functions_with_mismatching_arity_but_any_is_an_optional_param() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;

    let mut fixture = IsSubtypeFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: (number?) -> ()
        local b: (number) -> ()
        local c: (number, any) -> ()
    "#,
        ),
        None,
    );

    let a = fixture.base.require_type_string(&String::from("a"));
    let b = fixture.base.require_type_string(&String::from("b"));
    let c = fixture.base.require_type_string(&String::from("c"));

    assert!(!fixture.is_subtype(b, a));
    assert!(!fixture.is_subtype(c, a));
    assert!(fixture.is_subtype(a, b));
}
