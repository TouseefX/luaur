//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5683:type_infer_tables_bigger_nested_table_causes_big_type_error`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function main (tests/main.cpp)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_bigger_nested_table_causes_big_type_error

#[cfg(test)]
#[test]
fn type_infer_tables_bigger_nested_table_causes_big_type_error() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type File = {
            type: "file",
            name: string,
            content: string?,
        }

        type Dir = {
            type: "dir",
            name: string,
            children: { File | Dir }?,
        }


        type DirectoryChildren = { File | Dir }

        local newtree: DirectoryChildren = {
            {
                type = "dir",
                name = "src",
                children = {
                    {
                        type = "file",
                        path = "main.luau", -- I accidentally assign "path" instead of "name", causing a huge scary TypeError
                    }
                }
            }
        }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Table type '{ path: string, type: \"file\" }' not compatible with type 'File' because the former is missing field 'name'";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
    assert_eq!(
        Location::new(Position::new(21, 20), Position::new(24, 21)),
        result.errors[0].location
    );
}
