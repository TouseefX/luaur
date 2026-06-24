//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:245:variant_move_with_copyable_alternative`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record MoveOnly (tests/Variant.test.cpp)
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item variant_move_with_copyable_alternative

#[cfg(test)]
#[test]
fn variant_move_with_copyable_alternative() {
    use crate::records::move_only::MoveOnly;
    use alloc::string::String;
    use luaur_common::records::variant::Variant2;

    let mut v1: Variant2<String, MoveOnly> = Variant2::V0(String::from(
        "Hello, world! I am longer than a normal hello world string to avoid SSO.",
    ));
    let moved = core::mem::take(v1.get_if_0_mut().unwrap());
    let v2: Variant2<String, MoveOnly> = Variant2::V0(moved);

    let s1 = v1.get_if_0();
    assert!(s1.is_some());
    assert_eq!(s1.unwrap(), "");

    let s2 = v2.get_if_0();
    assert!(s2.is_some());
    assert_eq!(
        s2.unwrap(),
        "Hello, world! I am longer than a normal hello world string to avoid SSO."
    );
}
