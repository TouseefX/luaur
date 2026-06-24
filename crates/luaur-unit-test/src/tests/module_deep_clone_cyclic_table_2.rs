//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:148:module_deep_clone_cyclic_table_2`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method Property::setType (Analysis/src/Type.cpp)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item module_deep_clone_cyclic_table_2

#[cfg(test)]
#[test]
fn module_deep_clone_cyclic_table_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let mut src = TypeArena::default();

    let table_ty = src.add_type(TableType::table_type());
    let arg_pack = src.add_type_pack_initializer_list_type_id(&[]);
    let ret_pack = src.add_type_pack_initializer_list_type_id(&[table_ty]);
    let method_ty = src.add_type(FunctionType::function_type_new(
        arg_pack, ret_pack, None, false,
    ));

    let tt = unsafe { get_mutable_type_id::<TableType>(table_ty).as_mut() }
        .expect("expected source table type");
    tt.props
        .entry(String::from("get"))
        .or_default()
        .set_type(method_ty);

    let mut dest = TypeArena::default();

    let mut clone_state = CloneState::new(fixture.get_builtins());
    let clone_ty = clone_type(table_ty, &mut dest, &mut clone_state);
    let ctt = unsafe { get_mutable_type_id::<TableType>(clone_ty).as_mut() }
        .expect("expected cloned table type");

    let cloned_method_type = ctt
        .props
        .get("get")
        .and_then(|prop| prop.read_ty)
        .expect("expected cloned get method type");

    let cmf = unsafe { get_type_id::<FunctionType>(cloned_method_type).as_ref() }
        .expect("expected cloned get method function type");

    let clone_method_return_type =
        first(cmf.ret_types(), true).expect("expected cloned method return type");

    assert!(clone_method_return_type == clone_ty);
}
