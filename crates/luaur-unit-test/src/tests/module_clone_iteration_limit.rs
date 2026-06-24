//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:337:module_clone_iteration_limit`
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
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias ErrorType (Analysis/include/Luau/Type.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item module_clone_iteration_limit

#[cfg(test)]
#[test]
fn module_clone_iteration_limit() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::error_type::ErrorType;
    use luaur_common::FInt;

    let _sfi = ScopedFastInt::new(&FInt::LuauTypeCloneIterationLimit, 2000);
    let mut fixture = Fixture::fixture_bool(false);

    let mut src = TypeArena::default();

    let table = src.add_type(TableType::table_type());
    let mut nested = table;

    let nesting = 2500;
    for _ in 0..nesting {
        let child = src.add_type(TableType::table_type());
        let ttv = unsafe { get_mutable_type_id::<TableType>(nested).as_mut() }
            .expect("expected nested table type");
        ttv.props.insert("a".into(), Property::rw_type_id(child));
        nested = child;
    }

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let ty = clone_type(table, &mut dest, &mut clone_state);
    assert!(!unsafe { get_type_id::<ErrorType>(ty) }.is_null());

    // Cloning it again is an important test.
    let ty2 = clone_type(table, &mut dest, &mut clone_state);
    assert!(!unsafe { get_type_id::<ErrorType>(ty2) }.is_null());
}
