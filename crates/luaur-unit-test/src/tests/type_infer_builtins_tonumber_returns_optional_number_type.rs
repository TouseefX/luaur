//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1025:type_infer_builtins_tonumber_returns_optional_number_type`
//! Source: `tests/TypeInfer.builtins.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.builtins.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.builtins.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> macro tonumber (VM/src/lvm.h)
//!   - translates_to -> rust_item type_infer_builtins_tonumber_returns_optional_number_type

#[cfg(test)]
#[test]
fn type_infer_builtins_tonumber_returns_optional_number_type() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local b: number = tonumber('asdf')
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Expected this to be 'number', but got 'number?'; \n\
the 2nd component of the union is `nil`, which is not a subtype of `number`"
    } else {
        "Expected this to be 'number', but got 'number?'"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
