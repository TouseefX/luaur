//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:68:variant_emplace`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> method Variant::emplace (Common/include/Luau/Variant.h)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item variant_emplace

#[cfg(test)]
#[test]
fn variant_emplace() {
    use crate::records::bar::Bar;
    use luaur_common::records::variant::Variant2;

    Bar::reset_count();
    {
        let mut v1: Variant2<i32, Bar> = Variant2::default();

        assert_eq!(0, Bar::count());
        v1 = Variant2::V0(5);
        assert_eq!(5, *v1.get_if_0().unwrap());

        assert_eq!(0, Bar::count());

        v1 = Variant2::V1(Bar::new(11));
        assert_eq!(22, v1.get_if_1().unwrap().prop);
        assert_eq!(1, Bar::count());
    }

    assert_eq!(0, Bar::count());
}
