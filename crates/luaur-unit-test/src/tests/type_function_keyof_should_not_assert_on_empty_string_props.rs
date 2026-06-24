//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1723:type_function_keyof_should_not_assert_on_empty_string_props`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_keyof_should_not_assert_on_empty_string_props

#[cfg(test)]
#[test]
fn type_function_keyof_should_not_assert_on_empty_string_props() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    fixture.base.load_definition(
        &String::from(
            r#"
        declare class Foobar
            one: boolean
            [""]: number
        end
    "#,
        ),
        false,
    );

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type FoobarKeys = keyof<Foobar>;
        export type TableKeys = keyof<{ [""]: string, two: boolean }>
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "\"\" | \"one\"",
        to_string_type_id(fixture.base.require_type_alias(&String::from("FoobarKeys")))
    );
    assert_eq!(
        "\"\" | \"two\"",
        to_string_type_id(fixture.base.require_type_alias(&String::from("TableKeys")))
    );
}
