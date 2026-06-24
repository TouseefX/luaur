//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.test.cpp:1207:type_infer_type_infer_recursion_limit_no_ice`
//! Source: `tests/TypeInfer.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> enum Code (Config/include/Luau/LinterConfig.h)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - translates_to -> rust_item type_infer_type_infer_recursion_limit_no_ice

#[cfg(test)]
#[test]
fn type_infer_type_infer_recursion_limit_no_ice() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::{FFlag, FInt};

    let _recursion_limit = ScopedFastInt::new(&FInt::LuauTypeInferRecursionLimit, 2);

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function complex()
          function _(l0:t0): (any, ()->())
              return 0,_
          end
          type t0 = t0 | {}
          _(nil)
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Type contains a self-recursive construct that cannot be resolved",
            to_string_type_error(&result.errors[0])
        );
    } else {
        assert_eq!(
            "Code is too complex to typecheck! Consider simplifying the code around this area",
            to_string_type_error(&result.errors[0])
        );
    }
}
