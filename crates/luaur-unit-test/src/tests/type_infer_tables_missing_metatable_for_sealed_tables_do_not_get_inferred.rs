//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1620:type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred`
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
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record MetatableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred

#[cfg(test)]
#[test]
fn type_infer_tables_missing_metatable_for_sealed_tables_do_not_get_inferred() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::metatable_type::MetatableType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_mismatch::TypeMismatch;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {x = 1}

        local a = {x = 1}
        local b = {__index = {y = 2}}
        setmetatable(a, b)

        t = a
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let a = fixture.base.require_type_string(&String::from("a"));
    let t = fixture.base.require_type_string(&String::from("t"));
    assert_ne!(to_string_type_id(a), to_string_type_id(t));

    let tm = type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!(t, tm.wanted_type);
    assert_eq!(a, tm.given_type);

    unsafe { get_type_id::<MetatableType>(a).as_ref() }.unwrap_or_else(|| {
        panic!(
            "expected metatable type for a, got {}",
            to_string_type_id(a)
        );
    });
    unsafe { get_type_id::<TableType>(t).as_ref() }.unwrap_or_else(|| {
        panic!("expected table type for t, got {}", to_string_type_id(t));
    });
}
