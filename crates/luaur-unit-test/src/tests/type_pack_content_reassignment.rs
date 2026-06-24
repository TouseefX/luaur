//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypePack.test.cpp:200:type_pack_content_reassignment`
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
//!   - type_ref -> type_alias ErrorTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_pack_content_reassignment

#[cfg(test)]
#[test]
fn type_pack_content_reassignment() {
    use luaur_analysis::functions::as_mutable_type_pack::as_mutable;
    use luaur_analysis::functions::get_type_pack::get_type_pack_id;
    use luaur_analysis::records::free_type_pack::FreeTypePack;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::records::type_pack_var::TypePackVar;
    use luaur_analysis::type_aliases::error_type_pack::ErrorTypePack;
    use luaur_analysis::type_aliases::type_pack_variant::TypePackVariant;

    let my_error =
        TypePackVar::new_with_persistence(TypePackVariant::Error(ErrorTypePack::new()), true);

    let mut arena = TypeArena::default();

    let future_error = arena.add_type_pack_t(FreeTypePack::new(TypeLevel::default()));
    unsafe {
        (*as_mutable(future_error)).reassign(&my_error);
    }

    assert!(!unsafe { get_type_pack_id::<ErrorTypePack>(future_error) }.is_null());
    assert!(!unsafe { (*future_error).is_persistent() });
    assert_eq!(
        unsafe { (*future_error).owning_arena() },
        &mut arena as *mut TypeArena
    );
}
