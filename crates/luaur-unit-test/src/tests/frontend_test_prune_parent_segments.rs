//! Ported from `tests/Frontend.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:833:frontend_test_prune_parent_segments`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item frontend_test_prune_parent_segments

#[cfg(test)]
#[test]
fn frontend_test_prune_parent_segments() {
    use crate::functions::path_expr_to_module_name_fixture::path_expr_to_module_name_module_name_vector_string_view;

    assert_eq!(
        Some("Modules/Enum/ButtonState".to_string()),
        path_expr_to_module_name_module_name_vector_string_view(
            "",
            &vec![
                "Modules",
                "LuaApp",
                "DeprecatedDarkTheme",
                "Parent",
                "Parent",
                "Enum",
                "ButtonState",
            ],
        )
    );
    assert_eq!(
        Some("workspace/Foo/Bar/Baz".to_string()),
        path_expr_to_module_name_module_name_vector_string_view(
            "workspace/Foo/Quux",
            &vec!["script", "Parent", "Bar", "Baz"],
        )
    );
    assert_eq!(
        None,
        path_expr_to_module_name_module_name_vector_string_view("", &vec![])
    );
    assert_eq!(
        Some("script".to_string()),
        path_expr_to_module_name_module_name_vector_string_view("", &vec!["script"])
    );
    assert_eq!(
        Some("script/Parent".to_string()),
        path_expr_to_module_name_module_name_vector_string_view("", &vec!["script", "Parent"])
    );
    assert_eq!(
        Some("script".to_string()),
        path_expr_to_module_name_module_name_vector_string_view(
            "",
            &vec!["script", "Parent", "Parent"],
        )
    );
    assert_eq!(
        Some("script".to_string()),
        path_expr_to_module_name_module_name_vector_string_view(
            "",
            &vec!["script", "Test", "Parent"]
        )
    );
    assert_eq!(
        Some("script/Parent".to_string()),
        path_expr_to_module_name_module_name_vector_string_view(
            "",
            &vec!["script", "Test", "Parent", "Parent"],
        )
    );
    assert_eq!(
        Some("script/Parent".to_string()),
        path_expr_to_module_name_module_name_vector_string_view(
            "",
            &vec!["script", "Test", "Parent", "Test", "Parent", "Parent",],
        )
    );
}
