//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:447:type_infer_tables_open_table_unification_3`
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
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_open_table_unification_3

#[cfg(test)]
#[test]
fn type_infer_tables_open_table_unification_3() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id(x)
            return x
        end

        function foo(o)
            id(o.bar)
            id(o.baz)
        end
    "#,
        ),
        None,
    );

    let foo_type = fixture.require_type_string(&String::from("foo"));
    let foo_fn =
        unsafe { get_type_id::<FunctionType>(foo_type).as_ref() }.expect("expected FunctionType");

    let (foo_args, _) = flatten_type_pack_id(foo_fn.arg_types());
    assert_eq!(1, foo_args.len());

    let arg0 = foo_args[0];
    let arg0_table = unsafe { get_type_id::<TableType>(follow_type_id(arg0)).as_ref() }
        .expect("expected TableType");

    assert!(arg0_table.props.contains_key("bar"));
    assert!(arg0_table.props.contains_key("baz"));
}
