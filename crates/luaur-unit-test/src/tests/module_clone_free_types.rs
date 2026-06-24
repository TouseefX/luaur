//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:276:module_clone_free_types`
//! Source: `tests/Module.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Module.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Clone.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Module.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record FreeTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item module_clone_free_types

#[cfg(test)]
#[test]
fn module_clone_free_types() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::clone_clone::clone as clone_pack;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::fresh_type::fresh_type;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::get_type_pack::get_type_pack_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::free_type_pack::FreeTypePack;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::records::type_pack_var::TypePackVar;

    let mut fixture = Fixture::fixture_bool(false);
    let mut arena = TypeArena::default();
    let free_ty = fresh_type(
        &mut arena,
        fixture.get_builtins(),
        core::ptr::null_mut(),
        Polarity::Unknown,
    );
    let free_tp = TypePackVar::from(FreeTypePack::new(TypeLevel::default()));
    let free_tp_id = &free_tp as *const TypePackVar;

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned_ty = clone_type(free_ty, &mut dest, &mut clone_state);
    assert!(!unsafe { get_type_id::<FreeType>(cloned_ty) }.is_null());

    clone_state = CloneState::new(fixture.get_builtins());
    let cloned_tp = clone_pack(free_tp_id, &mut dest, &mut clone_state);
    assert!(!unsafe { get_type_pack_id::<FreeTypePack>(cloned_tp) }.is_null());
}
