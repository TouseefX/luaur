//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.const.test.cpp:131:type_infer_const_assign_different_values_to_const_x`
//! Source: `tests/TypeInfer.const.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.const.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.const.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record SyntaxError (Analysis/include/Luau/Error.h)
//!   - type_ref -> record Variable (Compiler/src/ValueTracking.h)
//!   - translates_to -> rust_item type_infer_const_assign_different_values_to_const_x

#[cfg(test)]
#[test]
fn type_infer_const_assign_different_values_to_const_x() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::syntax_error::SyntaxError;
    use luaur_common::FFlag;

    let _const2 = ScopedFastFlag::new(&FFlag::LuauConst2, true);
    let _export_value = ScopedFastFlag::new(&FFlag::LuauExportValueSyntax, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        const x: string? = nil
        local a = x
        x = "hello!"
        local b = x
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<SyntaxError>(&result.errors[0]).as_ref() }
        .expect("expected SyntaxError");
    assert_eq!(
        "Variable 'x' is constant and may not be reassigned",
        err.message()
    );
    assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "string?",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
}
