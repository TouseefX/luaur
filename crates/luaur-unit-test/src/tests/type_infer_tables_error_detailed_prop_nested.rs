//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2422:type_infer_tables_error_detailed_prop_nested`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_tables_error_detailed_prop_nested

#[cfg(test)]
#[test]
fn type_infer_tables_error_detailed_prop_nested() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type AS = { x: number, y: number }
type BS = { x: number, y: string }

type A = { a: boolean, b: AS }
type B = { a: boolean, b: BS }

local a: A = { a = false, b = { x = 123, y = 456 } }
local b: B = a
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be 'B', but got 'A'; \naccessing `b.y` results in `number` in the latter type and `string` in the former type, and `number` is not exactly `string`";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        let expected = r#"Expected this to be exactly 'B', but got 'A'
caused by:
  Property 'b' is not compatible.
Expected this to be exactly 'BS', but got 'AS'
caused by:
  Property 'y' is not compatible.
Expected this to be exactly 'string', but got 'number'"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
