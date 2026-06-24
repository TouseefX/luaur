//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1162:type_infer_builtins_table_freeze_is_generic`
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
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_builtins_table_freeze_is_generic

#[cfg(test)]
#[test]
fn type_infer_builtins_table_freeze_is_generic() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t1: {a: number} = {a = 42}
        local t2: {b: string} = {b = "hello"}
        local t3: {boolean} = {false, true}

        local tf1 = table.freeze(t1)
        local tf2 = table.freeze(t2)
        local tf3 = table.freeze(t3)

        local a = tf1.a
        local b = tf2.b
        local c = tf3[2]

        local d = tf1.b

        local a2 = t1.a
        local b2 = t2.b
        local c2 = t3[2]
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected_error = if !FFlag::DebugLuauForceOldSolver.get() {
        "Key 'b' not found in table '{ read a: number }'"
    } else {
        "Key 'b' not found in table '{ a: number }'"
    };
    assert_eq!(expected_error, to_string_type_error(&result.errors[0]));
    assert_eq!(
        Location {
            begin: Position {
                line: 13,
                column: 18,
            },
            end: Position {
                line: 13,
                column: 23,
            },
        },
        result.errors[0].location
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "{ read a: number }",
            to_string_type_id(fixture.base.require_type_at_position_position(Position {
                line: 15,
                column: 19,
            }))
        );
        assert_eq!(
            "{ read b: string }",
            to_string_type_id(fixture.base.require_type_at_position_position(Position {
                line: 16,
                column: 19,
            }))
        );
        assert_eq!(
            "{boolean}",
            to_string_type_id(fixture.base.require_type_at_position_position(Position {
                line: 17,
                column: 19,
            }))
        );
    }

    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("c")))
    );
    let expected_d = if !FFlag::DebugLuauForceOldSolver.get() {
        "any"
    } else {
        "*error-type*"
    };
    assert_eq!(
        expected_d,
        to_string_type_id(fixture.base.require_type_string(&String::from("d")))
    );
    assert_eq!(
        "number",
        to_string_type_id(fixture.base.require_type_string(&String::from("a2")))
    );
    assert_eq!(
        "string",
        to_string_type_id(fixture.base.require_type_string(&String::from("b2")))
    );
    assert_eq!(
        "boolean",
        to_string_type_id(fixture.base.require_type_string(&String::from("c2")))
    );
}
