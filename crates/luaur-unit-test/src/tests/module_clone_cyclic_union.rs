//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:368:module_clone_cyclic_union`
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
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item module_clone_cyclic_union

#[cfg(test)]
#[test]
fn module_clone_cyclic_union() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().number_type();
    let string_type = fixture.get_builtins().string_type();

    let mut src = TypeArena::default();
    let u = src.add_type(UnionType {
        options: alloc::vec![number_type, string_type],
    });
    let uu =
        unsafe { get_mutable_type_id::<UnionType>(u).as_mut() }.expect("expected source union");

    uu.options.push(u);

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned = clone_type(u, &mut dest, &mut clone_state);
    assert!(!cloned.is_null());

    let cloned_union =
        unsafe { get_type_id::<UnionType>(cloned).as_ref() }.expect("expected cloned union");
    assert_eq!(3, cloned_union.options.len());

    assert_eq!(number_type, cloned_union.options[0]);
    assert_eq!(string_type, cloned_union.options[1]);
    assert_eq!(cloned, cloned_union.options[2]);
}
