//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1262:type_infer_type_infer_cache_limit_normalizer`
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
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item type_infer_type_infer_cache_limit_normalizer

#[cfg(test)]
#[test]
fn type_infer_type_infer_cache_limit_normalizer() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FInt;

    let _normalize_cache_limit = ScopedFastInt::new(&FInt::LuauNormalizeCacheLimit, 10);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x : ((number) -> number) & ((string) -> string) & ((nil) -> nil) & (({}) -> {})
        local y : (number | string | nil | {}) -> (number | string | nil | {}) = x
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "Code is too complex to typecheck! Consider simplifying the code around this area",
        to_string_type_error(&result.errors[0])
    );
}
