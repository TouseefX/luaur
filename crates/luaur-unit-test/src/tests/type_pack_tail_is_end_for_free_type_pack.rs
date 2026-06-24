//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:121:type_pack_tail_is_end_for_free_type_pack`
//! Source: `tests/TypePack.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypePack.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypePack.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
//!   - translates_to -> rust_item type_pack_tail_is_end_for_free_type_pack

#[cfg(test)]
#[test]
fn type_pack_tail_is_end_for_free_type_pack() {
    use crate::records::type_pack_fixture::TypePackFixture;
    use luaur_analysis::functions::begin_type_pack::begin;
    use luaur_analysis::functions::end_type_pack::end;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let type_pack = fixture.fresh_type_pack();

    let mut it = begin(type_pack);
    let end_it = end(type_pack);
    while it.operator_ne(&end_it) {
        it.operator_inc();
    }

    assert_eq!(it.tail(), Some(type_pack));
}
