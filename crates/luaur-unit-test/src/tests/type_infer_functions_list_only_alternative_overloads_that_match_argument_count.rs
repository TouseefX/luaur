//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:239:type_infer_functions_list_only_alternative_overloads_that_match_argument_count`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record MultipleNonviableOverloads (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record ExtraInformation (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_list_only_alternative_overloads_that_match_argument_count

#[cfg(test)]
#[test]
fn type_infer_functions_list_only_alternative_overloads_that_match_argument_count() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::extra_information::ExtraInformation;
    use luaur_analysis::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local multiply: ((number)->number) & ((number)->string) & ((number, number)->number)
        multiply("")
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let mno = type_error_data_ref::<MultipleNonviableOverloads>(&result.errors[0])
            .expect("expected MultipleNonviableOverloads");
        assert_eq!(1, mno.attempted_arg_count());
    } else {
        let tm =
            type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
        assert_eq!(
            unsafe { (*fixture.builtin_types).number_type() },
            tm.wanted_type
        );
        assert_eq!(
            unsafe { (*fixture.builtin_types).string_type() },
            tm.given_type
        );
    }

    let ei = type_error_data_ref::<ExtraInformation>(&result.errors[1])
        .expect("expected ExtraInformation");

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Available overloads: (number) -> number; and (number) -> string",
            ei.message()
        );
    } else {
        assert_eq!(
            "Other overloads are also not viable: (number) -> string",
            ei.message()
        );
    }
}
