//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:1356:type_function_rawget_type_function_works_w_index_metatables`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - calls -> method Fixture::requireTypeAlias (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_function_rawget_type_function_works_w_index_metatables

#[cfg(test)]
#[test]
fn type_function_rawget_type_function_works_w_index_metatables() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local exampleClass = { Foo = "text", Bar = true }
        local exampleClass2 = setmetatable({ Foo = 8 }, { __index = exampleClass })
        type exampleTy2 = rawget<typeof(exampleClass2), "Foo">
        local function ok(idx: exampleTy2): number return idx end
        local exampleClass3 = setmetatable({ Bar = 5 }, { __index = exampleClass })
        type nilType = rawget<typeof(exampleClass3), "Foo">
        type numberType = rawget<typeof(exampleClass3), "Bar" | "Foo">
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        "nil",
        to_string_type_id(fixture.base.require_type_alias(&String::from("nilType")))
    );
    assert_eq!(
        "number?",
        to_string_type_id(fixture.base.require_type_alias(&String::from("numberType")))
    );
}
