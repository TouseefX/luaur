//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3196:type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UninhabitedTypeFunction (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar

#[cfg(test)]
#[test]
fn type_infer_tables_dont_crash_when_setmetatable_does_not_produce_a_metatabletypevar() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::uninhabited_type_function::UninhabitedTypeFunction;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
        .base
        .check_string_optional_frontend_options(&String::from("local x = setmetatable({})"), None);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        type_error_data_ref::<UninhabitedTypeFunction>(&result.errors[0])
            .expect("expected UninhabitedTypeFunction");
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Argument count mismatch. Function 'setmetatable' expects 2 arguments, but only 1 is specified",
            to_string_type_error(&result.errors[0])
        );
    }
}
