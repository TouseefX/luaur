//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2111:type_infer_functions_concrete_functions_are_not_supertypes_of_function`
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
//!   - calls -> function registerHiddenTypes (tests/Fixture.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_concrete_functions_are_not_supertypes_of_function

#[cfg(test)]
#[test]
fn type_infer_functions_concrete_functions_are_not_supertypes_of_function() {
    use crate::functions::register_hidden_types::register_hidden_types;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    let mut fixture = Fixture::fixture_bool(false);
    register_hidden_types(fixture.get_frontend());

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a: fun = function() end

        function one(arg: () -> ()) end
        function two(arg: <T>(T) -> T) end

        one(a)
        two(a)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    assert_eq!(6, result.errors[0].location.begin.line);
    let tm1 = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("() -> ()", to_string_type_id(tm1.wanted_type));
    assert_eq!("function", to_string_type_id(tm1.given_type));

    assert_eq!(7, result.errors[1].location.begin.line);
    let tm2 = unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("<T>(T) -> T", to_string_type_id(tm2.wanted_type));
    assert_eq!("function", to_string_type_id(tm2.given_type));
}
