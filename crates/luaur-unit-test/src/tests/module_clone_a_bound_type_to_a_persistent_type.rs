//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:509:module_clone_a_bound_type_to_a_persistent_type`
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
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - translates_to -> rust_item module_clone_a_bound_type_to_a_persistent_type

#[cfg(test)]
#[test]
fn module_clone_a_bound_type_to_a_persistent_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::follow_type::follow;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::bound_type::BoundType;

    let mut fixture = BuiltinsFixture::default();
    let mut arena = TypeArena::default();
    let bound_to = arena.add_type(BoundType {
        boundTo: fixture.base.get_builtins().numberType,
    });

    assert!(unsafe { (*fixture.base.get_builtins().numberType).persistent });

    let mut dest = TypeArena::default();
    let mut state = CloneState::new(fixture.base.get_builtins());
    let res = clone_type(bound_to, &mut dest, &mut state);

    assert_eq!(res, unsafe { follow(bound_to) });
}
