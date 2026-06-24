//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:80:module_dont_clone_persistent_primitive`
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
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item module_dont_clone_persistent_primitive

#[cfg(test)]
#[test]
fn module_dont_clone_persistent_primitive() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let number_type = fixture.get_builtins().number_type();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    // numberType is persistent. We leave it as-is.
    let new_number = clone_type(number_type, &mut dest, &mut clone_state);
    assert_eq!(new_number, number_type);
}
