//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:46:type_infer_functions_overload_resolution`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_functions_overload_resolution

#[cfg(test)]
#[test]
fn type_infer_functions_overload_resolution() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = (number) -> string
        type B = (string) -> number

        local function foo(f: A & B)
            return f(1), f("five")
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let t = fixture.require_type_string(&String::from("foo"));
    let foo_type = unsafe { get_type_id::<FunctionType>(t).as_ref() };
    assert!(foo_type.is_some(), "expected function type");
    assert_eq!(
        "(((number) -> string) & ((string) -> number)) -> (string, number)",
        to_string_type_id(t)
    );
}
