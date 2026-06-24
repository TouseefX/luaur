//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:745:type_infer_type_packs_type_alias_default_mixed_self`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_mixed_self

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_default_mixed_self() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Y<T, U = T, V... = ...number, W... = (T, U, V...)> = { a: (T, U, V...) -> W... }
local a: Y<number>
local b: Y<number, string>
local c: Y<number, string, ...boolean>
local d: Y<number, string, ...boolean, ...() -> ()>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    for (name, expected) in [
        (
            "a",
            "Y<number, number, ...number, (number, number, ...number)>",
        ),
        (
            "b",
            "Y<number, string, ...number, (number, string, ...number)>",
        ),
        (
            "c",
            "Y<number, string, ...boolean, (number, string, ...boolean)>",
        ),
        ("d", "Y<number, string, ...boolean, ...() -> ()>"),
    ] {
        assert_eq!(
            expected,
            to_string_type_id(fixture.require_type_string(&String::from(name))),
            "{name}"
        );
    }
}
