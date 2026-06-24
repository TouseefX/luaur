//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:703:type_infer_tables_indexers_get_quantified_too`
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
//!   - calls -> method NativeModuleRef::swap (CodeGen/src/SharedCodeAllocator.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableIndexer (Analysis/include/Luau/Type.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_indexers_get_quantified_too

#[cfg(test)]
#[test]
fn type_infer_tables_indexers_get_quantified_too() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::flatten_type_pack::flatten_type_pack_id;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function swap(p)
            local temp = p[0]
            p[0] = p[1]
            p[1] = temp
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "<a>({a}) -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("swap")))
        );
    } else {
        let ftv = unsafe {
            get_type_id::<FunctionType>(fixture.require_type_string(&String::from("swap"))).as_ref()
        }
        .expect("expected FunctionType");

        let (arg_vec, _) = flatten_type_pack_id(ftv.arg_types());
        assert_eq!(1, arg_vec.len());

        let ttv = unsafe { get_type_id::<TableType>(follow_type_id(arg_vec[0])).as_ref() }
            .expect("expected TableType");
        let indexer = ttv.indexer.as_ref().expect("expected table indexer");

        assert_eq!("number", to_string_type_id(indexer.index_type));

        let index_result_type = unsafe { follow_type_id(indexer.index_result_type) };
        let generic = unsafe { get_type_id::<GenericType>(index_result_type).as_ref() };
        assert!(
            generic.is_some(),
            "Expected generic but got {}",
            to_string_type_id(index_result_type)
        );
    }
}
