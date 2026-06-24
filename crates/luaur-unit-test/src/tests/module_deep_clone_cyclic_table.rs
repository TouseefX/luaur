//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:107:module_deep_clone_cyclic_table`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item module_deep_clone_cyclic_table

#[cfg(test)]
#[test]
fn module_deep_clone_cyclic_table() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Cyclic = {}
        function Cyclic.get()
            return Cyclic
        end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("Cyclic"));

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());
    let clone_ty = clone_type(ty, &mut dest, &mut clone_state);

    let ttv = unsafe { get_mutable_type_id::<TableType>(clone_ty).as_mut() }
        .expect("expected cloned table type");

    assert_eq!(Some(String::from("Cyclic")), ttv.synthetic_name.clone());

    let method_type = ttv
        .props
        .get("get")
        .and_then(|prop| prop.read_ty)
        .expect("expected get method type");

    let ftv = unsafe { get_type_id::<FunctionType>(method_type).as_ref() }
        .expect("expected get method function type");

    let method_return_type = first(ftv.ret_types(), true).expect("expected method return type");

    assert!(
        method_return_type == clone_ty,
        "{} should be pointer identical to {}",
        to_string_type_id(method_type),
        to_string_type_id(clone_ty)
    );
    assert_eq!(2, dest.type_packs.size());
    assert_eq!(2, dest.types.size());
}
