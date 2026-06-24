//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:583:type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash`
//! Source: `tests/TypeInfer.typeInstantiations.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typeInstantiations.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typeInstantiations.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record UnknownSymbol (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_typeof_in_method_call_type_args_no_crash() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::type_aliases::type_error_data::TypeErrorData;
    use luaur_common::FFlag;

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _visit_call_type_args = ScopedFastFlag::new(&FFlag::LuauVisitCallTypeArgsInDfg, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}
        function t:f<T, U>() end

        local x = 5
        globl = 42

        t:f<<typeof(x), string>>()
        t:f<<number, typeof(x)>>()
        t:f<<typeof(globl), unknown>>()
        t:f<<typeof(t.f), unknown>>()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(matches!(
        result.errors[0].data,
        TypeErrorData::UnknownSymbol(_)
    ));
}
