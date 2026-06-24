//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Variant.test.cpp:239:variant_move`
//! Source: `tests/Variant.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Variant.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Variant.h
//! - incoming:
//!   - declares <- source_file tests/Variant.test.cpp
//! - outgoing:
//!   - type_ref -> record MoveOnly (tests/Variant.test.cpp)
//!   - translates_to -> rust_item variant_move

#[cfg(test)]
#[test]
fn variant_move() {
    use crate::records::move_only::MoveOnly;
    use luaur_common::records::variant::Variant1;

    let v1: Variant1<MoveOnly> = Variant1::V0(MoveOnly);
    let _v2 = v1;
}
