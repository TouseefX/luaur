//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.user.test.cpp:1058:type_function_user_udtf_type_overrides_eq_metamethod`
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
//!   - calls -> method SubtypeFixture::idx (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item type_function_user_udtf_type_overrides_eq_metamethod

#[cfg(test)]
#[test]
fn type_function_user_udtf_type_overrides_eq_metamethod() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type function hello()
            local p1 = types.string
            local p2 = types.string
            local t1 = types.newtable(nil, nil, nil)
            t1:setproperty(types.singleton("string"), types.boolean)
            t1:setmetatable(t1)
            local t2 = types.newtable(nil, nil, nil)
            t2:setproperty(types.singleton("string"), types.boolean)
            t1:setmetatable(t1)
            if p1 == p2 and t1 == t2 then
                return types.number
            end
            return types.unknown
        end
        local function ok(idx: hello<>): number return idx end
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
