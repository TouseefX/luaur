//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:841:type_infer_generics_error_detailed_function_mismatch_generic_pack`
//! Source: `tests/TypeInfer.generics.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.generics.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.generics.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record GenericTypePackCountMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_generics_error_detailed_function_mismatch_generic_pack

#[cfg(test)]
#[test]
fn type_infer_generics_error_detailed_function_mismatch_generic_pack() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::generic_type_pack_count_mismatch::GenericTypePackCountMismatch;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type C = () -> ()
type D = <T...>() -> ()

local c: C
local d: D = c
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);

        let generic_mismatch =
            type_error_data_ref::<GenericTypePackCountMismatch>(&result.errors[0])
                .expect("expected GenericTypePackCountMismatch");
        assert_eq!(1, generic_mismatch.sub_ty_generic_pack_count());
        assert_eq!(0, generic_mismatch.super_ty_generic_pack_count());

        let mismatch =
            type_error_data_ref::<TypeMismatch>(&result.errors[1]).expect("expected TypeMismatch");
        assert_eq!("() -> ()", to_string_type_id(mismatch.given_type));
        assert_eq!("<T...>() -> ()", to_string_type_id(mismatch.wanted_type));
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);

        let mismatch =
            type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
        assert_eq!(
            "different number of generic type pack parameters",
            mismatch.reason
        );
    }
}
