//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1843:type_infer_functions_too_few_arguments_variadic_generic`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_too_few_arguments_variadic_generic

#[cfg(test)]
#[test]
fn type_infer_functions_too_few_arguments_variadic_generic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::records::count_mismatch::CountMismatch;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
function test(a: number, b: string, ...)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let acm = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() }
        .expect("expected CountMismatch");

    assert_eq!(3, acm.expected());
    assert_eq!(1, acm.actual());
    assert_eq!(CountMismatch::Arg, acm.context());
    assert!(acm.is_variadic());
}
