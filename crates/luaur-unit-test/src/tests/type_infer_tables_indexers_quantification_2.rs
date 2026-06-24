//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:740:type_infer_tables_indexers_quantification_2`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_indexers_quantification_2

#[cfg(test)]
#[test]
fn type_infer_tables_indexers_quantification_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function mergesort(arr)
            local p = arr[0]
            return arr
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ftv = unsafe {
        get_type_id::<FunctionType>(fixture.require_type_string(&String::from("mergesort")))
            .as_ref()
    }
    .expect("expected FunctionType");

    let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
    assert_eq!(1, arg_vec.len());
    let arg_type = unsafe { get_type_id::<TableType>(follow_type_id(arg_vec[0])).as_ref() }
        .expect("expected argument TableType");

    let (ret_vec, _) = flatten_type_pack_id(ftv.ret_types());
    assert_eq!(1, ret_vec.len());
    let ret_type = unsafe { get_type_id::<TableType>(follow_type_id(ret_vec[0])).as_ref() }
        .expect("expected return TableType");

    assert_eq!(arg_type.state, ret_type.state);
    assert_eq!(arg_vec[0], ret_vec[0]);
}
