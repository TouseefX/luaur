//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1497:type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored`
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
//!   - calls -> function gmatch (VM/src/lstrlib.cpp)
//!   - type_ref -> record CountMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored

#[cfg(test)]
#[test]
fn type_infer_builtins_gmatch_capture_types_parens_in_sets_are_ignored() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::count_mismatch::CountMismatch;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a, b, c = string.gmatch("T(his)() is a string", "(T[()])()")()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let mismatch = unsafe { get_type_error::<CountMismatch>(&result.errors[0]).as_ref() }
        .expect("expected CountMismatch");
    assert_eq!(2, mismatch.expected());
    assert_eq!(3, mismatch.actual());
    assert_eq!(
        "string?",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
}
