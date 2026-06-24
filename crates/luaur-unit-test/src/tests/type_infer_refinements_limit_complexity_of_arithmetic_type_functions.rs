//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.refinements.test.cpp:2780:type_infer_refinements_limit_complexity_of_arithmetic_type_functions`
//! Source: `tests/TypeInfer.refinements.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.refinements.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.refinements.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_refinements_limit_complexity_of_arithmetic_type_functions

#[cfg(test)]
#[test]
fn type_infer_refinements_limit_complexity_of_arithmetic_type_functions() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local Hermite = {}

        function Hermite:__init(p0, p1, m0, m1)
            self[1] = {
                p0.x;
                p0.y;
                p0.z;
            }
            self[2] = {
                m0.x;
                m0.y;
                m0.z;
            }
            self[3] = {
                3*(p1.x - p0.x) - 2*m0.x - m1.x;
                3*(p1.y - p0.y) - 2*m0.y - m1.y;
                3*(p1.z - p0.z) - 2*m0.z - m1.z;
            }
        end

        return Hermite
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
}
