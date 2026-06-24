//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:636:type_infer_builtins_string_format_tostring_specifier`
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
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - translates_to -> rust_item type_infer_builtins_string_format_tostring_specifier

#[cfg(test)]
#[test]
fn type_infer_builtins_string_format_tostring_specifier() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        string.format("%* %* %* %*", "string", 1, true, function() end)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
