//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:569:type_infer_extern_types_optional_class_casts_work_in_new_solver`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_extern_types_optional_class_casts_work_in_new_solver

#[cfg(test)]
#[test]
fn type_infer_extern_types_optional_class_casts_work_in_new_solver() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = { x: ChildClass }
        type B = { x: BaseClass }

        local a = { x = ChildClass.New() } :: A
        local opt_a = a :: A?
        local b = { x = BaseClass.New() } :: B
        local opt_b = b :: B?
        local b_from_a = a :: B
        local b_from_opt_a = opt_a :: B
        local opt_b_from_a = a :: B?
        local opt_b_from_opt_a = opt_a :: B?
        local a_from_b = b :: A
        local a_from_opt_b = opt_b :: A
        local opt_a_from_b = b :: A?
        local opt_a_from_opt_b = opt_b :: A?
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
