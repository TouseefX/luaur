//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:163:type_pack_follows_bound_type_packs`
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
//!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
//!   - type_ref -> record Bound (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_pack_follows_bound_type_packs

#[cfg(test)]
#[test]
fn type_pack_follows_bound_type_packs() {
    use crate::functions::collect_type_pack::collect_type_pack;
    use crate::records::type_pack_fixture::TypePackFixture;
    use luaur_analysis::functions::as_mutable_type_pack::as_mutable;
    use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

    let mut fixture = TypePackFixture::type_pack_fixture();
    let tail_tp = fixture.new_type_pack(alloc::vec![fixture.types[2], fixture.types[3]], None);
    let middle_tp = fixture.fresh_type_pack();
    unsafe {
        (*as_mutable(middle_tp)).operator_assign_type_pack_variant(TypePackVariant::Bound(tail_tp));
    }
    let head_tp = fixture.new_type_pack(alloc::vec![], Some(middle_tp));

    let count = collect_type_pack(head_tp).len();

    assert_eq!(2, count);
}
