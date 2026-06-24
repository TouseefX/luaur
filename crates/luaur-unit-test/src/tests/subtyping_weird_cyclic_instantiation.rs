//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1909:subtyping_weird_cyclic_instantiation`
//! Source: `tests/Subtyping.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Subtyping.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypePath.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/RegisterCallbacks.h
//! - incoming:
//!   - declares <- source_file tests/Subtyping.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
//!   - type_ref -> record DenseHashMap (Common/include/Luau/DenseHash.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item subtyping_weird_cyclic_instantiation

#[cfg(test)]
#[test]
fn subtyping_weird_cyclic_instantiation() {
    use crate::records::subtype_fixture::SubtypeFixture;
    use alloc::{string::String, vec};
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::instantiate_2_instantiation_2::instantiate_2;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::type_id::TypeId;
    use luaur_analysis::type_aliases::type_pack_id::TypePackId;
    use luaur_common::records::dense_hash_map::DenseHashMap;

    let mut fixture = SubtypeFixture::default();
    let mut arena = TypeArena::default();
    let mut scope = Scope::scope_type_pack_id(fixture.builtin_types.anyTypePack);

    let generic_t = arena.add_type(GenericType::generic_type_name_polarity(
        &String::from("T"),
        Polarity::Mixed,
    ));
    let id_arg_types = arena.add_type_pack_initializer_list_type_id(&[generic_t]);
    let id_ret_types = arena.add_type_pack_initializer_list_type_id(&[generic_t]);
    let id_ty = arena.add_type(FunctionType::new_with_generics(
        vec![generic_t],
        vec![],
        id_arg_types,
        id_ret_types,
        None,
        false,
    ));

    let mut generic_substitutions: DenseHashMap<TypeId, TypeId> =
        DenseHashMap::new(core::ptr::null());
    let generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId> =
        DenseHashMap::new(core::ptr::null());

    let free_ty = arena.fresh_type_not_null_builtin_types_scope(&fixture.builtin_types, &mut scope);
    let ft = unsafe { get_mutable_type_id::<FreeType>(free_ty).as_mut() };
    let ft = ft.expect("fresh type should be a FreeType");
    ft.lower_bound = id_ty;
    ft.upper_bound = fixture.builtin_types.unknownType;

    *generic_substitutions.get_or_insert(generic_t) = free_ty;

    assert_eq!("<T>(T) -> T", to_string_type_id(id_ty));

    let res = instantiate_2(
        &mut arena,
        generic_substitutions,
        generic_pack_substitutions,
        &mut *fixture.subtyping,
        &mut scope,
        id_ty,
    );

    assert_eq!("<T>(T) -> T", to_string_type_id(id_ty));

    let res = res.expect("instantiate2 should return a type");
    assert_eq!("<T>(T) -> T", to_string_type_id(res));
}
