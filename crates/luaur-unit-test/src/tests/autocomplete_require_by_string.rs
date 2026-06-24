//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3800:autocomplete_require_by_string`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - type_ref -> record RequireCompletion (tests/Autocomplete.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias AutocompleteEntryMap (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record AutocompleteResult (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item autocomplete_require_by_string

#[cfg(test)]
#[test]
fn autocomplete_require_by_string() {
    use crate::functions::null_callback_autocomplete_test::null_callback;
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
    use luaur_ast::records::position::Position;

    fn check_entries(entry_map: &AutocompleteEntryMap, completions: &[(&str, &str)]) {
        assert_eq!(completions.len(), entry_map.len());

        for (label, insert_text) in completions {
            let entry = entry_map
                .get(*label)
                .unwrap_or_else(|| panic!("missing require completion `{label}`"));
            assert_eq!(entry.insert_text.as_deref(), Some(*insert_text));
        }
    }

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.base.file_resolver.source.insert(
        String::from("MainModule"),
        String::from(
            r#"
        local info = "MainModule serves as the root directory"
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("MainModule/Folder"),
        String::from(
            r#"
        local info = "MainModule/Folder serves as a subdirectory"
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("MainModule/Folder/Requirer"),
        String::from(
            r#"
        local res0 = require("@")

        local res1 = require(".")
        local res2 = require("./")
        local res3 = require("./Sib")

        local res4 = require("..")
        local res5 = require("../")
        local res6 = require("../Sib")
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("MainModule/Folder/SiblingDependency"),
        String::from(
            r#"
        return {"result"}
    "#,
        ),
    );

    fixture.base.base.file_resolver.source.insert(
        String::from("MainModule/ParentDependency"),
        String::from(
            r#"
        return {"result"}
    "#,
        ),
    );

    let module_name = String::from("MainModule/Folder/Requirer");

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 1,
                column: 31,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("@defaultalias", "@defaultalias"),
            ("./", "./"),
            ("../", "../"),
        ],
    );

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 3,
                column: 31,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("@defaultalias", "@defaultalias"),
            ("./", "./"),
            ("../", "../"),
        ],
    );

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 4,
                column: 32,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("..", "."),
            ("Requirer", "./Requirer"),
            ("SiblingDependency", "./SiblingDependency"),
        ],
    );

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 5,
                column: 35,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("..", "."),
            ("Requirer", "./Requirer"),
            ("SiblingDependency", "./SiblingDependency"),
        ],
    );

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 7,
                column: 32,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("@defaultalias", "@defaultalias"),
            ("./", "./"),
            ("../", "../"),
        ],
    );

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 8,
                column: 33,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("..", "../.."),
            ("Folder", "../Folder"),
            ("ParentDependency", "../ParentDependency"),
        ],
    );

    let ac_result = fixture
        .base
        .autocomplete_module_name_position_string_completion_callback(
            &module_name,
            Position {
                line: 9,
                column: 36,
            },
            Box::new(null_callback),
        );
    check_entries(
        &ac_result.entry_map,
        &[
            ("..", "../.."),
            ("Folder", "../Folder"),
            ("ParentDependency", "../ParentDependency"),
        ],
    );
}
