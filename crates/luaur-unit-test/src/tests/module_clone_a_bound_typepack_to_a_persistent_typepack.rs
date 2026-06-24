//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:523:module_clone_a_bound_typepack_to_a_persistent_typepack`
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
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> type_alias BoundTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - translates_to -> rust_item module_clone_a_bound_typepack_to_a_persistent_typepack

#[cfg(test)]
#[test]
fn module_clone_a_bound_typepack_to_a_persistent_typepack() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::clone_clone::clone as clone_pack;
    use luaur_analysis::functions::follow_type_pack::follow;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::bound_type_pack::BoundTypePack;

    let mut fixture = BuiltinsFixture::default();
    let mut arena = TypeArena::default();
    let bound_to = arena.add_type_pack_t(BoundTypePack {
        boundTo: fixture.base.get_builtins().neverTypePack,
    });

    assert!(unsafe { (*fixture.base.get_builtins().neverTypePack).is_persistent() });

    let mut dest = TypeArena::default();
    let mut state = CloneState::new(fixture.base.get_builtins());
    let res = clone_pack(bound_to, &mut dest, &mut state);

    assert_eq!(res, unsafe { follow(bound_to) });
}
