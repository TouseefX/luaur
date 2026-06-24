//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1017:type_infer_generics_generic_argument_count_too_many`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_generics_generic_argument_count_too_many

#[cfg(test)]
#[test]
fn type_infer_generics_generic_argument_count_too_many() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
function test2(a: number, b: string)
    return 1
end

function wrapper<A...>(f: (A...) -> number, ...: A...)
end

wrapper(test2, 1, "", 3)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let cm = type_error_data_ref::<CountMismatch>(&result.errors[0])
            .expect("expected CountMismatch");
        assert_eq!(3, cm.expected());
        assert_eq!(4, cm.actual());
        assert_eq!(CountMismatch::Arg, cm.context());
    } else {
        assert_eq!(
            "Argument count mismatch. Function 'wrapper' expects 3 arguments, but 4 are specified",
            to_string_type_error(&result.errors[0])
        );
    }
}
