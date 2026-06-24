//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:656:type_infer_type_packs_type_alias_default_type_self`
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
//!   - translates_to -> rust_item type_infer_type_packs_type_alias_default_type_self

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_alias_default_type_self() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Y<T, U = T> = { a: T, b: U }

local a: Y<number> = { a = 2, b = 3 }
local b: Y<string> = { a = "h", b = "s" }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Y<number, number>",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "Y<string, string>",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Y<T, U = (T, T) -> string> = { a: T, b: U }

local a: Y<number>
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Y<number, (number, number) -> string>",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
}
