//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:208:type_infer_singletons_enums_using_singletons_mismatch`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_singletons_enums_using_singletons_mismatch

#[cfg(test)]
#[test]
fn type_infer_singletons_enums_using_singletons_mismatch() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyEnum = "foo" | "bar" | "baz"
        local a : MyEnum = "bang"
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = r#"Expected this to be '"bar" | "baz" | "foo"', but got '"bang"'; 
this is because 
	 * the 1st component of the union is `"foo"`, and `"bang"` is not a subtype of `"foo"`
	 * the 2nd component of the union is `"bar"`, and `"bang"` is not a subtype of `"bar"`
	 * the 3rd component of the union is `"baz"`, and `"bang"` is not a subtype of `"baz"`"#;
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        assert_eq!(
            r#"Expected this to be '"bar" | "baz" | "foo"', but got '"bang"'; none of the union options are compatible"#,
            to_string_type_error(&result.errors[0])
        );
    }
}
