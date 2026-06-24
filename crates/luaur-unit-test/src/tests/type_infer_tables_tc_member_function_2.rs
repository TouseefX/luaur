//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:278:type_infer_tables_tc_member_function_2`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item type_infer_tables_tc_member_function_2

#[cfg(test)]
#[test]
fn type_infer_tables_tc_member_function_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from("local T = {U={}}  function T.U:foo() return 5 end"),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let table_type = unsafe {
        get_type_id::<TableType>(fixture.require_type_string(&String::from("T"))).as_ref()
    }
    .expect("expected table type");

    let u_type = table_type
        .props
        .get("U")
        .and_then(|prop| prop.read_ty)
        .expect("expected U read type");
    let u_table = unsafe { get_type_id::<TableType>(u_type).as_ref() }.expect("expected U table");

    let foo_ty = u_table
        .props
        .get("foo")
        .and_then(|prop| prop.read_ty)
        .expect("expected foo read type");
    let method_type = unsafe { get_type_id::<FunctionType>(follow_type_id(foo_ty)).as_ref() }
        .expect("expected function type");

    let (method_args, _) = flatten_type_pack_id(method_type.arg_types());
    assert_eq!(1, method_args.len());
}
