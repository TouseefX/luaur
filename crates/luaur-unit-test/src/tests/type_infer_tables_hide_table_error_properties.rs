//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2099:type_infer_tables_hide_table_error_properties`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_tables_hide_table_error_properties

#[cfg(test)]
#[test]
fn type_infer_tables_hide_table_error_properties() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict

        local function f()
        local function mkt() return { x = 1 } end
        local t = mkt()

        function t.a() end
        function t.b() end

        return t
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Cannot add property 'a' to table '{ x: number }'",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Cannot add property 'b' to table '{ x: number }'",
        to_string_type_error(&result.errors[1])
    );
}
