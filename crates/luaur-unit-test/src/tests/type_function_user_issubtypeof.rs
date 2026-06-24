//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:3180:type_function_user_issubtypeof`
//! Source: `tests/TypeFunction.user.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.user.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.user.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UserDefinedTypeFunctionError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_function_user_issubtypeof

#[cfg(test)]
#[test]
fn type_function_user_issubtypeof() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }
    let _sff = ScopedFastFlag::new(&FFlag::LuauUdtfTypeIsSubtypeOf, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function checksubtype(a, b)
            if not a:issubtypeof(b) then
                error("Not a subtype!")
            end
            return a
        end

        local x: checksubtype<nil, nil>                          -- S
        local y: checksubtype<nil, string?>                      -- S
        local z: checksubtype<"Hello", string>                   -- S
        local w: checksubtype<string | vector | number, number>  -- F
        local a: checksubtype<boolean, number>                   -- F
        local b: checksubtype<false, nil>                        -- F
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    for error in &result.errors {
        assert!(
            matches!(&error.data, TypeErrorData::UserDefinedTypeFunctionError(_)),
            "expected UserDefinedTypeFunctionError, got {:?}",
            error
        );
    }
    assert_eq!(11, result.errors[0].location.begin.line);
    assert_eq!(12, result.errors[1].location.begin.line);
    assert_eq!(13, result.errors[2].location.begin.line);
}
