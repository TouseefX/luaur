//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:143:type_infer_builtins_sort_with_bad_predicate`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_builtins_sort_with_bad_predicate

#[cfg(test)]
#[test]
fn type_infer_builtins_sort_with_bad_predicate() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local t = {'one', 'two', 'three'}
        local function p(a: number, b: number) return a < b end
        table.sort(t, p)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t'((string, string) -> boolean)?'\nbut got\n\t'(number, number) -> boolean'\ncaused by:\n  None of the union options are compatible. For example:\nExpected this to be\n\t'(string, string) -> boolean'\nbut got\n\t'(number, number) -> boolean'\ncaused by:\n  Argument #1 type is not compatible.\nExpected this to be 'number', but got 'string'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
