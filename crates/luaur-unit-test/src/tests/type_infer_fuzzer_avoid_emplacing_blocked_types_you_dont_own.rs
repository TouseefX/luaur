//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:2847:type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own`
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
//!   - translates_to -> rust_item type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own

#[cfg(test)]
#[test]
fn type_infer_fuzzer_avoid_emplacing_blocked_types_you_dont_own() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        if if _ then _ else nil then
            local l0 = require(module0)
            _ = l0
        elseif _ then
            function _(l0:true,...)
            end
        else
        end
        _ = l0
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local l0 = require(module0)
        local l10 = require(module0)
        do end
        for l0=_,_,true do
        end
        do
        local l0 = require(module0)
        _ = l0
        local l10 = require(module0)
        function _()
        end
        end
        local l10 = require(module0)
    "#,
        ),
        None,
    );
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
