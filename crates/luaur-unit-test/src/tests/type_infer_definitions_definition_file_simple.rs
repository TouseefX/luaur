//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:17:type_infer_definitions_definition_file_simple`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function getGlobalBinding (Analysis/src/BuiltinDefinitions.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_definitions_definition_file_simple

#[cfg(test)]
#[test]
fn type_infer_definitions_definition_file_simple() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_global_binding::get_global_binding;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare foo: number
        declare function bar(x: number): string
        declare foo2: typeof(foo)
    "#,
        ),
        false,
    );

    let frontend = fixture.get_frontend();
    let global_foo_ty = get_global_binding(&mut frontend.globals, "foo");
    assert_eq!("number", to_string_type_id(global_foo_ty));

    let global_bar_ty = get_global_binding(&mut frontend.globals, "bar");
    assert_eq!("(number) -> string", to_string_type_id(global_bar_ty));

    let global_foo2_ty = get_global_binding(&mut frontend.globals, "foo2");
    assert_eq!("number", to_string_type_id(global_foo2_ty));

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: number = foo - 1
        local y: string = bar(x)
        local z: number | string = x
        z = y
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
