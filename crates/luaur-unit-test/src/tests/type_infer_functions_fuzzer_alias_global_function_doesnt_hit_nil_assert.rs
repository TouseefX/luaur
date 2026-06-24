//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2829:type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert`
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
//!   - translates_to -> rust_item type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert

#[cfg(test)]
#[test]
fn type_infer_functions_fuzzer_alias_global_function_doesnt_hit_nil_assert() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
function _()
end
local function l0()
    function _()
    end
end
_ = _
"#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
