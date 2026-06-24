//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.definitions.test.cpp:693:type_infer_definitions_extern_read_write_dual_attribute`
//! Source: `tests/TypeInfer.definitions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.definitions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.definitions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function write (tests/JsonEmitter.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_definitions_extern_read_write_dual_attribute

#[cfg(test)]
#[test]
fn type_infer_definitions_extern_read_write_dual_attribute() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare extern type dual_attribute with
            read value: boolean
            write value: number
        end
    "#,
        ),
        false,
    );

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local da: dual_attribute
local x: boolean = da.value
local y: number = da.value
da.value = 5
da.value = false
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
        result.errors[0].data,
        TypeErrorData::TypeMismatch(_)
    ));
    assert!(matches!(
        result.errors[1].data,
        TypeErrorData::TypeMismatch(_)
    ));
    assert_eq!(4, result.errors[0].location.begin.line);
    assert_eq!(6, result.errors[1].location.begin.line);
}
