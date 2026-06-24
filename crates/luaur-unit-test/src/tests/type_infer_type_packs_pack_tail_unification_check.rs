//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:918:type_infer_type_packs_pack_tail_unification_check`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_type_packs_pack_tail_unification_check

#[cfg(test)]
#[test]
fn type_infer_type_packs_pack_tail_unification_check() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local a: () -> (number, ...string)
local b: () -> (number, ...boolean)
a = b
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Expected this to be\n\t'() -> (number, ...string)'\nbut got\n\t'() -> (number, ...boolean)'; \nit returns a tail of the variadic `boolean` in the latter type and `string` in the former type, and `boolean` is not a subtype of `string`"
    } else {
        "Expected this to be\n\t'() -> (number, ...string)'\nbut got\n\t'() -> (number, ...boolean)'\ncaused by:\n  Expected this to be 'string', but got 'boolean'"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
