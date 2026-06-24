//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:605:type_infer_extern_types_indexable_extern_types`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method SubtypeFixture::str (tests/Subtyping.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Index (Analysis/include/Luau/TypePath.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function format (tests/StringUtils.test.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_indexable_extern_types

#[cfg(test)]
#[test]
fn type_infer_extern_types_indexable_extern_types() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let mut check = |source: &str| {
        fixture
            .base
            .base
            .check_string_optional_frontend_options(&String::from(source), None)
    };

    let result = check(
        r#"
            local x : IndexableClass
            local y = x.stringKey
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            local y = x["stringKey"]
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            local str : string
            local y = x[str]            -- Index with a non-const string
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            local y = x[7]              -- Index with a numeric key
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            x.stringKey = 42
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            x["stringKey"] = 42
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            local str : string
            x[str] = 42                 -- Index with a non-const string
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            x[1] = 42                   -- Index with a numeric key
        "#,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let result = check(
        r#"
            local x : IndexableClass
            local y = x[true]
        "#,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be 'number | string', but got 'boolean';\n\
this is because\n\
\t* the 1st component of the union is `string`, and `boolean` is not a subtype of `string`\n\
\t* the 2nd component of the union is `number`, and `boolean` is not a subtype of `number`\n";
        crate::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
        assert_eq!(
            "Expected this to be 'number | string', but got 'boolean'; none of the union options are compatible",
            to_string_type_error(&result.errors[0])
        );
    }

    let result = check(
        r#"
            local x : IndexableClass
            x[true] = 42
        "#,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be 'number | string', but got 'boolean';\n\
this is because\n\
\t * the 1st component of the union is `string`, and `boolean` is not a subtype of `string`\n\
\t * the 2nd component of the union is `number`, and `boolean` is not a subtype of `number`\n";
        crate::CHECK_LONG_STRINGS_EQ!(expected, to_string_type_error(&result.errors[0]));
    } else {
        assert_eq!(
            "Expected this to be 'number | string', but got 'boolean'; none of the union options are compatible",
            to_string_type_error(&result.errors[0])
        );
    }

    let result = check(
        r#"
            local x : IndexableClass
            x.key = "string value"
        "#,
    );

    if FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
    }

    let result = check(
        r#"
            local x : IndexableClass
            local str : string = x.key
        "#,
    );

    assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[0])
    );

    let result = check(
        r#"
            local x : IndexableNumericKeyClass
            x.key = 1
        "#,
    );
    assert_eq!(
        "Key 'key' not found in external type 'IndexableNumericKeyClass'",
        to_string_type_error(&result.errors[0])
    );

    let result = check(
        r#"
            local x : IndexableNumericKeyClass
            x["key"] = 1
        "#,
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Key 'key' not found in external type 'IndexableNumericKeyClass'",
            to_string_type_error(&result.errors[0])
        );
    } else {
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
    }

    let result = check(
        r#"
            local x : IndexableNumericKeyClass
            local str : string
            x[str] = 1                  -- Index with a non-const string
        "#,
    );

    assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
    );

    let result = check(
        r#"
            local x : IndexableNumericKeyClass
            local y = x.key
        "#,
    );
    assert_eq!(
        "Key 'key' not found in external type 'IndexableNumericKeyClass'",
        to_string_type_error(&result.errors[0])
    );

    let result = check(
        r#"
            local x : IndexableNumericKeyClass
            local y = x["key"]
        "#,
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Key 'key' not found in external type 'IndexableNumericKeyClass'",
            to_string_type_error(&result.errors[0])
        );
    } else {
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
    }

    let result = check(
        r#"
            local x : IndexableNumericKeyClass
            local str : string
            local y = x[str]            -- Index with a non-const string
        "#,
    );

    assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
    );
}
