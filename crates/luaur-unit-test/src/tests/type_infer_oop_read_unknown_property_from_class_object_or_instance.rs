//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1205:type_infer_oop_read_unknown_property_from_class_object_or_instance`
//! Source: `tests/TypeInfer.oop.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.oop.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.oop.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_oop_read_unknown_property_from_class_object_or_instance

#[cfg(test)]
#[test]
fn type_infer_oop_read_unknown_property_from_class_object_or_instance() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::unknown_property::UnknownProperty;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _tidy = ScopedFastFlag::new(&FFlag::LuauTidyTypePrototyping, true);
    let _access_violation = ScopedFastFlag::new(&FFlag::LuauTweakAccessViolationReporting, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        class Point
            public x: number
            public y: number

            function zero()
                return Point {x=0, y=0}
            end
        end

        local p = Point.zero()
        local a = p.z
        local b = Point.z
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    let up0 = type_error_data_ref::<UnknownProperty>(&result.errors[0])
        .expect("expected UnknownProperty");
    assert_eq!("z", up0.key());
    let up1 = type_error_data_ref::<UnknownProperty>(&result.errors[1])
        .expect("expected UnknownProperty");
    assert_eq!("z", up1.key());
}
