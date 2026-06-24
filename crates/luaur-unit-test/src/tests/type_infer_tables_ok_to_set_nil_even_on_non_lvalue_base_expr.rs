//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1906:type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record Location (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr

#[cfg(test)]
#[test]
fn type_infer_tables_ok_to_set_nil_even_on_non_lvalue_base_expr() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(): { [string]: number }
            return { ["foo"] = 1 }
        end

        f()["foo"] = nil
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(
            t: {known_prop: boolean, [string]: number},
            key: string
        )
            t[key] = nil
            t["hello"] = nil
            t.undefined = nil
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(t: {known_prop: boolean, [string]: number, })
            t.known_prop = nil
        end
    "#,
        ),
        None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 27
            },
            end: Position {
                line: 2,
                column: 30
            },
        },
        result.errors[0].location
    );
    assert_eq!(
        "Expected this to be 'boolean', but got 'nil'",
        to_string_type_error(&result.errors[0])
    );

    fixture.load_definition(
        &String::from(
            r#"
        declare class FancyHashtable
            [string]: number
            real_property: string
        end
     "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function removekey(fh: FancyHashtable, other_key: string)
            fh["hmmm"] = nil
            fh[other_key] = nil
            fh.dne = nil
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function removekey(fh: FancyHashtable)
            fh.real_property = nil
        end
    "#,
        ),
        None,
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 2,
                column: 31
            },
            end: Position {
                line: 2,
                column: 34
            },
        },
        result.errors[0].location
    );
    assert_eq!(
        "Expected this to be 'string', but got 'nil'",
        to_string_type_error(&result.errors[0])
    );
}
