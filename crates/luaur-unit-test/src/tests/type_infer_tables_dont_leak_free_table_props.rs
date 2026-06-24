//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3536:type_infer_tables_dont_leak_free_table_props`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_tables_dont_leak_free_table_props

#[cfg(test)]
#[test]
fn type_infer_tables_dont_leak_free_table_props() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_common::FFlag;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function a(state)
            print(state.blah)
        end

        local function b(state) -- The bug was that we inferred state: {blah: any, gwar: any}
            print(state.gwar)
        end

        return function()
            return function(state)
                a(state)
                b(state)
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "({ read blah: unknown }) -> ()",
            to_string_type_id(fixture.base.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "({ read gwar: unknown }) -> ()",
            to_string_type_id(fixture.base.require_type_string(&String::from("b")))
        );
        let module = unsafe { &*fixture.base.get_main_module(false) };
        assert_eq!(
            "(...any) -> ({ read blah: unknown, read gwar: unknown }) -> ()",
            to_string_type_pack_id(module.return_type)
        );
    } else {
        assert_eq!(
            "<a>({+ blah: a +}) -> ()",
            to_string_type_id(fixture.base.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "<a>({+ gwar: a +}) -> ()",
            to_string_type_id(fixture.base.require_type_string(&String::from("b")))
        );
        let module = unsafe { &*fixture.base.get_main_module(false) };
        assert_eq!(
            "() -> <a, b>({+ blah: a, gwar: b +}) -> ()",
            to_string_type_pack_id(module.return_type)
        );
    }
}
