//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:129:variant_equality`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item variant_equality

#[cfg(test)]
#[test]
fn variant_equality() {
    use alloc::string::String;
    use luaur_common::records::variant::Variant2;

    let v1: Variant2<i32, String> = Variant2::V1(String::from("hi"));
    let v2: Variant2<i32, String> = Variant2::V1(String::from("me"));
    let v3: Variant2<i32, String> = Variant2::V0(1);
    let v4: Variant2<i32, String> = Variant2::V0(0);
    let v5: Variant2<i32, String> = Variant2::default();

    assert_eq!(v1, v1);
    assert_ne!(v1, v2);
    assert_ne!(v1, v3);
    assert_ne!(v3, v4);
    assert_eq!(v4, v5);
}
