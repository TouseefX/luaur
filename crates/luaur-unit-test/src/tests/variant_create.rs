//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:50:variant_create`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - translates_to -> rust_item variant_create

#[cfg(test)]
#[test]
fn variant_create() {
    use crate::records::foo::Foo;
    use luaur_common::records::variant::Variant2;

    let v1: Variant2<i32, Foo> = Variant2::V0(1);
    let v2: Variant2<Foo, i32> = Variant2::V0(Foo { x: 2 });

    let f = Foo { x: 3 };
    let v3: Variant2<Foo, i32> = Variant2::V0(f);

    assert!(v1.get_if_0().is_some());
    assert_eq!(*v1.get_if_0().unwrap(), 1);

    assert!(v2.get_if_0().is_some());
    assert_eq!(v2.get_if_0().unwrap().x, 2);

    assert!(v3.get_if_0().is_some());
    assert_eq!(v3.get_if_0().unwrap().x, 3);
}
