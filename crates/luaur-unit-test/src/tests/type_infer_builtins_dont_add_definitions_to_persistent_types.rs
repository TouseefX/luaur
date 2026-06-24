//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.builtins.test.cpp:1058:type_infer_builtins_dont_add_definitions_to_persistent_types`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_builtins_dont_add_definitions_to_persistent_types

#[cfg(test)]
#[test]
fn type_infer_builtins_dont_add_definitions_to_persistent_types() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
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
        local f = math.sin
        local function g(x) return math.sin(x) end
        f = g
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let f_type = fixture.base.require_type_string(&String::from("f"));
    let ftv = unsafe { get_type_id::<FunctionType>(f_type).as_ref() }.expect("expected function");
    assert!(unsafe { (*f_type).persistent });
    assert!(ftv.definition().is_none());

    let g_type = fixture.base.require_type_string(&String::from("g"));
    let gtv = unsafe { get_type_id::<FunctionType>(g_type).as_ref() }.expect("expected function");
    assert!(!unsafe { (*g_type).persistent });
    assert!(gtv.definition().is_some());
}
