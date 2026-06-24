//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:180:type_pack_post_and_pre_increment`
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
//!   - translates_to -> rust_item type_pack_post_and_pre_increment

#[cfg(test)]
#[test]
fn type_pack_post_and_pre_increment() {
    use crate::records::type_pack_fixture::TypePackFixture;
    use luaur_analysis::functions::begin_type_pack::begin;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let type_pack = fixture.new_type_pack(
        alloc::vec![
            fixture.types[0],
            fixture.types[1],
            fixture.types[2],
            fixture.types[3],
        ],
        None,
    );

    let mut it1 = begin(type_pack);
    let mut it2 = it1.operator_inc_i32();
    it2.operator_inc();
    let it3 = it2.clone();

    assert_eq!(*it2.operator_deref(), *it3.operator_deref());
}
