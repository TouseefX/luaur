//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:944:type_infer_tc_if_else_expressions_expected_type_1`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_tc_if_else_expressions_expected_type_1

#[cfg(test)]
#[test]
fn type_infer_tc_if_else_expressions_expected_type_1() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type X = {number | string}
local a: X = if true then {"1", 2, 3} else {4, 5, 6}
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{number | string}",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("a")),
            &mut opts
        )
    );
}
