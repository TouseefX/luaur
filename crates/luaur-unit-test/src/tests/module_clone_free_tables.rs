//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:293:module_clone_free_tables`
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
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item module_clone_free_tables

#[cfg(test)]
#[test]
fn module_clone_free_tables() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);

    let mut table_ty = Type::from(TableType::table_type());
    let table_ty_id = &mut table_ty as *mut Type as *const Type;
    let ttv = unsafe { get_mutable_type_id::<TableType>(table_ty_id).as_mut() }
        .expect("expected source table type");
    ttv.state = TableState::Free;

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned = clone_type(table_ty_id, &mut dest, &mut clone_state);
    let cloned_ttv =
        unsafe { get_type_id::<TableType>(cloned).as_ref() }.expect("expected cloned table type");

    assert_eq!(cloned_ttv.state, TableState::Free);
}
