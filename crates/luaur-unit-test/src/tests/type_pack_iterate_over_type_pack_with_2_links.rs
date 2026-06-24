//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:79:type_pack_iterate_over_type_pack_with_2_links`
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
//!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_pack_iterate_over_type_pack_with_2_links

#[cfg(test)]
#[test]
fn type_pack_iterate_over_type_pack_with_2_links() {
    use crate::functions::collect_type_pack::collect_type_pack;
    use crate::records::type_pack_fixture::TypePackFixture;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let type_pack1 = fixture.new_type_pack(alloc::vec![fixture.types[0], fixture.types[1]], None);
    let type_pack2 = fixture.new_type_pack(
        alloc::vec![fixture.types[0], fixture.types[3]],
        Some(type_pack1),
    );

    let result = collect_type_pack(type_pack2);

    assert_eq!(4, result.len());
    assert_eq!(fixture.types[0], result[0]);
    assert_eq!(fixture.types[3], result[1]);
    assert_eq!(fixture.types[0], result[2]);
    assert_eq!(fixture.types[1], result[3]);
}
