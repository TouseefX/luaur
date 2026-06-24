//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:90:module_deep_clone_non_persistent_primitive`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item module_deep_clone_non_persistent_primitive

#[cfg(test)]
#[test]
fn module_deep_clone_non_persistent_primitive() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::freeze::freeze;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::unfreeze::unfreeze;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::primitive_type::PrimitiveType;
    use luaur_analysis::records::primitive_type::Type;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    // Create a new number type that isn't persistent.
    let old_number = {
        let frontend = fixture.get_frontend();
        let global_types = frontend.globals.global_types_mut();
        unfreeze(global_types);
        let old_number = global_types.add_type(PrimitiveType {
            r#type: Type::Number,
            metatable: None,
        });
        freeze(global_types);
        old_number
    };

    let new_number = clone_type(old_number, &mut dest, &mut clone_state);

    assert_ne!(new_number, old_number);
    assert_eq!("number", to_string_type_id(old_number));
    assert_eq!("number", to_string_type_id(new_number));
    assert_eq!(1, dest.types.size());
}
