//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1448:type_infer_builtins_gmatch_capture_types_2`
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
//!   - translates_to -> rust_item type_infer_builtins_gmatch_capture_types_2

#[cfg(test)]
#[test]
fn type_infer_builtins_gmatch_capture_types2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a, b, c = ("This is a string"):gmatch("(.()(%a+))")()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_string(&String::from("c")))
    );
}
