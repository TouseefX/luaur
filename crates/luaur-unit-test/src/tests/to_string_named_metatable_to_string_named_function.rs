//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToString.test.cpp:160:to_string_named_metatable_to_string_named_function`
//! Source: `tests/ToString.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ToString.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/ToString.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item to_string_named_metatable_to_string_named_function

#[cfg(test)]
#[test]
fn to_string_named_metatable_to_string_named_function() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_named_function_to_string::to_string_named_function_string_function_type;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_common::FFlag;

    if !FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function createTbl(): NamedMetatable
            return setmetatable({}, {})
        end
        type NamedMetatable = typeof(createTbl())
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let ty = fixture.base.require_type_string(&String::from("createTbl"));
    let ftv = unsafe { get_type_id::<FunctionType>(follow_type_id(ty)).as_ref() }
        .expect("expected createTbl to be a function type");
    assert_eq!(
        "createTbl(): NamedMetatable",
        to_string_named_function_string_function_type("createTbl", ftv)
    );
}
