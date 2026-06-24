//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1299:type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden`
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
//!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden

#[cfg(test)]
#[test]
fn type_infer_oop_writes_to_unknown_class_instance_properties_are_forbidden() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::property_access_violation::{
        PropertyAccessViolation, PropertyAccessViolation_Context,
    };
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

            function magnitude(self): number
                return 5 -- stochastic approximation for performance
            end
        end

        local p = Point.zero()

        p.magnitude = function(p: Point) return 3 end
        p.zero = function() return Point { x = 1, y = 1 } end
        p.one = function() return Point { x = 1, y = 1 } end

        p.__index = {}
    "#,
        ),
        None,
    );

    let expected = ["magnitude", "zero", "one", "__index"];
    assert_eq!(expected.len(), result.errors.len(), "{:?}", result.errors);
    for (err, key) in result.errors.iter().zip(expected) {
        let pav = type_error_data_ref::<PropertyAccessViolation>(err)
            .expect("expected PropertyAccessViolation");
        assert_eq!(key, pav.key());
        assert_eq!(PropertyAccessViolation_Context::CannotWrite, pav.context());
    }
}
