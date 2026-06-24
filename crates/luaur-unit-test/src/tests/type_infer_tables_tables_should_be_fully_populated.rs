//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3924:type_infer_tables_tables_should_be_fully_populated`
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
//!   - type_ref -> record ToStringOptions (Analysis/include/Luau/ToString.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_tables_tables_should_be_fully_populated

#[cfg(test)]
#[test]
fn type_infer_tables_tables_should_be_fully_populated() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {
            x = 5 :: NonexistingTypeWhichEndsUpReturningAnErrorType,
            y = 5
        }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let mut options = ToStringOptions::default();
    options.exhaustive = true;
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "{ x: *error-type*, y: number }",
            to_string_type_id_to_string_options(
                fixture.base.require_type_string(&String::from("t")),
                &mut options,
            )
        );
    } else {
        assert_eq!(
            "{| x: *error-type*, y: number |}",
            to_string_type_id_to_string_options(
                fixture.base.require_type_string(&String::from("t")),
                &mut options,
            )
        );
    }
}
