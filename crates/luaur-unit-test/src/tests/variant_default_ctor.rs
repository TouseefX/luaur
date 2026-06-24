//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:37:variant_default_ctor`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - translates_to -> rust_item variant_default_ctor

#[cfg(test)]
#[test]
fn variant_default_ctor() {
    use crate::records::foo::Foo;
    use luaur_common::records::variant::Variant2;

    let v1: Variant2<i32, Foo> = Variant2::default();
    let v2: Variant2<Foo, i32> = Variant2::default();

    assert!(v1.get_if_0().is_some());
    assert_eq!(*v1.get_if_0().unwrap(), 0);
    assert!(v1.get_if_1().is_none());

    assert!(v2.get_if_0().is_some());
    assert_eq!(v2.get_if_0().unwrap().x, 42);
}
