//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:96:type_pack_get_the_tail`
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
//!   - calls -> method TypePackFixture::newTypePack (tests/TypePack.test.cpp)
//!   - translates_to -> rust_item type_pack_get_the_tail

#[cfg(test)]
#[test]
fn type_pack_get_the_tail() {
    use crate::records::type_pack_fixture::TypePackFixture;
    use luaur_analysis::functions::begin_type_pack::begin;
    use luaur_analysis::functions::end_type_pack::end;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let free_tail = fixture.fresh_type_pack();
    let type_pack = fixture.new_type_pack(alloc::vec![fixture.types[0]], Some(free_tail));

    let mut it = begin(type_pack);
    let end_it = end(type_pack);
    let mut count = 0;
    while it.operator_ne(&end_it) {
        count += 1;
        it.operator_inc();
    }

    assert_eq!(1, count);
    assert!(it.operator_eq(&end(type_pack)));
    assert_eq!(it.tail(), Some(free_tail));
}
