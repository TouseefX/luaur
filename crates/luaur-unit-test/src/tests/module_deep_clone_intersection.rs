//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:218:module_deep_clone_intersection`
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
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item module_deep_clone_intersection

#[cfg(test)]
#[test]
fn module_deep_clone_intersection() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());
    let number_type = fixture.get_builtins().number_type();
    let string_type = fixture.get_builtins().string_type();

    let old_intersection = {
        let frontend = fixture.get_frontend();
        let global_types = frontend.globals.global_types_mut();
        unfreeze(global_types);
        let old_intersection = global_types.add_type(IntersectionType {
            parts: alloc::vec![number_type, string_type],
        });
        freeze(global_types);
        old_intersection
    };

    let new_intersection = clone_type(old_intersection, &mut dest, &mut clone_state);

    assert_ne!(new_intersection, old_intersection);
    assert_eq!("number & string", to_string_type_id(new_intersection));
    assert_eq!(1, dest.types.size());
}
