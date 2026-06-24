//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:48:type_pack_type_pack_hello`
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
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - translates_to -> rust_item type_pack_type_pack_hello

#[cfg(test)]
#[test]
fn type_pack_type_pack_hello() {
    use crate::records::type_pack_fixture::TypePackFixture;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::records::type_pack_var::TypePackVar;

    let fixture = TypePackFixture::type_pack_fixture();
    let tp = TypePackVar::from(TypePack::new(
        alloc::vec![fixture.types[0], fixture.types[1]],
        None,
    ));

    assert!(tp.type_pack_var_operator_eq(&tp));
}
