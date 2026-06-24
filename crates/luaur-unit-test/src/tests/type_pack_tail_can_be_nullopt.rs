//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:113:type_pack_tail_can_be_nullopt`
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
//!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
//!   - translates_to -> rust_item type_pack_tail_can_be_nullopt

#[cfg(test)]
#[test]
fn type_pack_tail_can_be_nullopt() {
    use crate::records::type_pack_fixture::TypePackFixture;
    use luaur_analysis::functions::end_type_pack::end;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[0]], None);

    let it = end(type_pack);
    assert_eq!(None, it.tail());
}
