//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3797:type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly`
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
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - type_ref -> record MultipleNonviableOverloads (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly

#[cfg(test)]
#[test]
fn type_infer_tables_a_free_shape_can_turn_into_a_scalar_directly() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function stringByteList(str)
            local out = {}
            for i = 1, #str do
                table.insert(out, string.byte(str, i))
            end
            return table.concat(out, ",")
        end

        local x = stringByteList("xoo")
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(!result.errors.is_empty(), "{:?}", result.errors);
        assert!(
            result
                .errors
                .iter()
                .any(|error| type_error_data_ref::<MultipleNonviableOverloads>(error).is_some()),
            "{:?}",
            result.errors
        );
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
