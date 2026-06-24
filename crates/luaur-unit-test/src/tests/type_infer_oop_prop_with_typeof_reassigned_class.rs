//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:1103:type_infer_oop_prop_with_typeof_reassigned_class`
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
//!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
//!   - translates_to -> rust_item type_infer_oop_prop_with_typeof_reassigned_class

#[cfg(test)]
#[test]
fn type_infer_oop_prop_with_typeof_reassigned_class() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::syntax_error::SyntaxError;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);
    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let _export_value = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
class Animal end
Animal = nil
class l0
public _:typeof(Animal)
end
"#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = type_error_data_ref::<SyntaxError>(&result.errors[0]).expect("expected SyntaxError");
    assert_eq!(
        "Variable 'Animal' is constant and may not be reassigned",
        err.message()
    );
}
