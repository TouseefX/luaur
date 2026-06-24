//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:68:type_pack_iterate_over_type_pack`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_pack_iterate_over_type_pack

#[cfg(test)]
#[test]
fn type_pack_iterate_over_type_pack() {
    use crate::functions::collect_type_pack::collect_type_pack;
    use crate::records::type_pack_fixture::TypePackFixture;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[1]], None);

    let res = collect_type_pack(type_pack);

    assert_eq!(2, res.len());
}
