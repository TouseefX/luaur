//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:660:type_infer_provisional_intersection_of_functions_of_different_arities`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_infer_provisional_intersection_of_functions_of_different_arities

#[cfg(test)]
#[test]
fn type_infer_provisional_intersection_of_functions_of_different_arities() {
    use crate::records::is_subtype_fixture::IsSubtypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = IsSubtypeFixture::default();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = (any) -> ()
        type B = (any, any) -> ()
        type T = A & B

        local a: A
        local b: B
        local t: T
    "#,
        ),
        None,
    );

    let _a = fixture.base.require_type_string(&String::from("a"));
    let _b = fixture.base.require_type_string(&String::from("b"));

    assert_eq!(
        "((any) -> ()) & ((any, any) -> ())",
        to_string_type_id(fixture.base.require_type_string(&String::from("t")))
    );
}
