//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:464:type_infer_definitions_class_definition_malformed_string`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> record LoadDefinitionFileResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method Frontend::loadDefinitionFile (Analysis/src/Frontend.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method ParseError::getMessage (Ast/src/Parser.cpp)
//!   - calls -> method StringWriter::literal (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function escape (Common/src/StringUtils.cpp)
//!   - translates_to -> rust_item type_infer_definitions_class_definition_malformed_string

#[cfg(test)]
#[test]
fn type_infer_definitions_class_definition_malformed_string() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::frontend::Frontend;

    let mut fixture = Fixture::fixture_bool(false);

    let frontend_ptr = fixture.get_frontend() as *mut Frontend;
    let result = unsafe {
        let target_scope = (*frontend_ptr).globals.global_scope();
        (*frontend_ptr).load_definition_file(
            &mut (*frontend_ptr).globals,
            target_scope,
            r#"
        declare class Foo
            ["a\0property"]: string
        end
    "#,
            String::from("@test"),
            false,
            false,
        )
    };

    assert!(!result.success);
    assert_eq!(1, result.parse_result.errors.len());
    assert_eq!(
        "String literal contains malformed escape sequence or \\0",
        result.parse_result.errors[0].get_message()
    );
}
