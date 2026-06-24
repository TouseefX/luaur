//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:55:type_pack_first_chases_bound_type_pack_vars`
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
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> type_alias BoundTypePack (Analysis/include/Luau/TypePack.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item type_pack_first_chases_bound_type_pack_vars

#[cfg(test)]
#[test]
fn type_pack_first_chases_bound_type_pack_vars() {
    use luaur_analysis::functions::first::first;
    use luaur_analysis::records::primitive_type::{PrimitiveType, Type as PrimitiveKind};
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::records::type_pack_var::TypePackVar;
    use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

    let nil_type = Type::from(PrimitiveType::primitive_type_type_item(
        PrimitiveKind::NilType,
    ));
    let nil_type_id = &nil_type as *const Type;

    let tp1 = TypePackVar::from(TypePack::new(alloc::vec![nil_type_id], None));
    let tp1_id = &tp1 as *const TypePackVar;

    let tp2 = TypePackVar::new(TypePackVariant::Bound(tp1_id));
    let tp2_id = &tp2 as *const TypePackVar;

    let tp3 = TypePackVar::from(TypePack::new(alloc::vec![], Some(tp2_id)));
    let tp3_id = &tp3 as *const TypePackVar;

    assert_eq!(first(tp3_id, true), Some(nil_type_id));
}
