//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2605:type_infer_tables_explicitly_typed_table_error`
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
//!   - translates_to -> rust_item type_infer_tables_explicitly_typed_table_error

#[cfg(test)]
#[test]
fn type_infer_tables_explicitly_typed_table_error() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
type Super = { x : number }
type Sub = { x : number, y: number }
type HasSuper = { p : Super }
type HasSub = { p : Sub }
local tmp = { p = { x = 5, y = 7 }}
local a: HasSuper = tmp
a.p = { x = 9 }
-- needs to be an error because
local y: number = tmp.p.y
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be 'HasSuper', but got 'tmp'; \naccessing `p` results in `{ x: number, y: number }` in the latter type and `Super` in the former type, and `{ x: number, y: number }` is not exactly `Super`";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        let expected = r#"Expected this to be exactly 'HasSuper', but got 'tmp'
caused by:
  Property 'p' is not compatible.
Table type '{| x: number, y: number |}' not compatible with type 'Super' because the former has extra field 'y'"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
