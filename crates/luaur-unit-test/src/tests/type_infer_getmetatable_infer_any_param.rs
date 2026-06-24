//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1880:type_infer_getmetatable_infer_any_param`
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
//!   - translates_to -> rust_item type_infer_getmetatable_infer_any_param

#[cfg(test)]
#[test]
fn type_infer_getmetatable_infer_any_param() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let _result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function check(x): any
            return getmetatable(x)
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(unknown) -> any",
            to_string_type_id(fixture.base.require_type_string(&String::from("check")))
        );
    } else {
        assert_eq!(
            "({ @metatable any, {+  +} }) -> any",
            to_string_type_id(fixture.base.require_type_string(&String::from("check")))
        );
    }
}
