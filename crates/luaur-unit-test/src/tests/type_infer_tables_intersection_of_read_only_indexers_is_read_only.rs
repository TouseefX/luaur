//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4757:type_infer_tables_intersection_of_read_only_indexers_is_read_only`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> function fail (Config/src/Config.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_intersection_of_read_only_indexers_is_read_only

#[cfg(test)]
#[test]
fn type_infer_tables_intersection_of_read_only_indexers_is_read_only() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::property_access_violation::{
        PropertyAccessViolation, PropertyAccessViolation_Context,
    };
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _read_only_indexers = ScopedFastFlag::new(&FFlag::LuauReadOnlyIndexers, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function readOk(t: {read [string]: number} & {read [string]: number | string})
            local _x: number = t["k"]
        end
        local function writeFails(t: {read [string]: number} & {read [string]: number | string})
            t["k"] = 1
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let av = type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
        .expect("expected PropertyAccessViolation");
    assert_eq!(
        "{ read [string]: number | string } & { read [string]: number }",
        to_string_type_id(av.table())
    );
    assert_eq!("k", av.key());
    assert_eq!(PropertyAccessViolation_Context::CannotWrite, av.context());
}
