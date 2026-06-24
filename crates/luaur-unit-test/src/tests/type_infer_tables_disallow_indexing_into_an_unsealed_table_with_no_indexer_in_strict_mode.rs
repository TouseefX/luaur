//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1026:type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode`
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
//!   - calls -> method Compiler::getConstant (Compiler/src/Compiler.cpp)
//!   - translates_to -> rust_item type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode

#[cfg(test)]
#[test]
fn type_infer_tables_disallow_indexing_into_an_unsealed_table_with_no_indexer_in_strict_mode() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local constants = {
            key1 = "value1",
            key2 = "value2"
        }

        function getConstant(key)
            return constants[key]
        end

        local k1 = getConstant("key1")
    "#,
        ),
        None,
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "unknown"
    } else {
        "any"
    };
    assert_eq!(
        expected,
        to_string_type_id(fixture.require_type_string(&String::from("k1")))
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
