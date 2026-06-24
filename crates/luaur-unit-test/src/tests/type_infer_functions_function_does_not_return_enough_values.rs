//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1121:type_infer_functions_function_does_not_return_enough_values`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypePackMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_functions_function_does_not_return_enough_values

#[cfg(test)]
#[test]
fn type_infer_functions_function_does_not_return_enough_values() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::count_mismatch::CountMismatch;
    use luaur_analysis::records::type_pack_mismatch::TypePackMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        function f(): (number, string)
            return 55
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let tpm = unsafe { get_type_error::<TypePackMismatch>(&result.errors[0]).as_ref() }
            .expect("expected TypePackMismatch");
        assert_eq!("number, string", to_string_type_pack_id(tpm.wanted_tp()));
        assert_eq!("number", to_string_type_pack_id(tpm.given_tp()));
    } else {
        let acm = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() }
            .expect("expected CountMismatch");
        assert_eq!(CountMismatch::Return, acm.context());
        assert_eq!(2, acm.expected());
        assert_eq!(1, acm.actual());
    }
}
