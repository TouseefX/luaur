//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:481:module_clone_table_bound_to_table_bound_to_table`
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
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item module_clone_table_bound_to_table_bound_to_table

#[cfg(test)]
#[test]
fn module_clone_table_bound_to_table_bound_to_table() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut fixture = BuiltinsFixture::default();
    let mut arena = TypeArena::default();

    let a = arena.add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Free,
        TypeLevel::default(),
        core::ptr::null_mut(),
    ));
    unsafe { get_mutable_type_id::<TableType>(a).as_mut() }
        .expect("expected table a")
        .name = Some("a".into());

    let b = arena.add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Free,
        TypeLevel::default(),
        core::ptr::null_mut(),
    ));
    unsafe { get_mutable_type_id::<TableType>(b).as_mut() }
        .expect("expected table b")
        .name = Some("b".into());

    let c = arena.add_type(TableType::table_type_table_state_type_level_scope(
        TableState::Free,
        TypeLevel::default(),
        core::ptr::null_mut(),
    ));
    unsafe { get_mutable_type_id::<TableType>(c).as_mut() }
        .expect("expected table c")
        .name = Some("c".into());

    unsafe { get_mutable_type_id::<TableType>(a).as_mut() }
        .expect("expected table a")
        .bound_to = Some(b);
    unsafe { get_mutable_type_id::<TableType>(b).as_mut() }
        .expect("expected table b")
        .bound_to = Some(c);

    let mut dest = TypeArena::default();
    let mut state = CloneState::new(fixture.base.get_builtins());
    let res = clone_type(a, &mut dest, &mut state);

    assert_eq!(1, dest.types.size());

    let table_a = unsafe { get_type_id::<TableType>(res).as_ref() }.expect("expected cloned table");
    assert_eq!(Some("c"), table_a.name.as_deref());
    assert!(table_a.bound_to.is_none());
}
