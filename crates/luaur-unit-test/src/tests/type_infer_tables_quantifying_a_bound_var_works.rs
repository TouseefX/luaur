//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2239:type_infer_tables_quantifying_a_bound_var_works`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_quantifying_a_bound_var_works

#[cfg(test)]
#[test]
fn type_infer_tables_quantifying_a_bound_var_works() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::metatable_type::MetatableType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local clazz = {}
        clazz.__index = clazz

        function clazz:speak()
            return "hi"
        end

        function clazz.new()
            return setmetatable({}, clazz)
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.base.require_type_string(&String::from("clazz"));
    let table = unsafe { get_type_id::<TableType>(ty).as_ref() }
        .unwrap_or_else(|| panic!("Expected a table but got {}", to_string_type_id(ty)));
    let prop = table.props.get("new").expect("expected new property");
    let read_ty = prop.read_ty.expect("expected new property read type");
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(read_ty)).as_ref() }
        .expect("expected FunctionType");

    let (returns, tail) = flatten_type_pack_id(unsafe { ftv.ret_types() });
    assert!(tail.is_none(), "expected finite return pack");
    assert_eq!(1, returns.len());

    let mtv = unsafe { get_type_id::<MetatableType>(follow_type_id(returns[0])).as_ref() }
        .expect("expected MetatableType");
    let returned_table = unsafe { get_type_id::<TableType>(follow_type_id(mtv.table())).as_ref() }
        .expect("expected returned TableType");
    assert_eq!(TableState::Sealed, returned_table.state);
}
