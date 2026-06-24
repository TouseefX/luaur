//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typeInstantiations.test.cpp:608:type_infer_type_instantiations_typeof_local_in_type_pack_no_crash`
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
//!   - translates_to -> rust_item type_infer_type_instantiations_typeof_local_in_type_pack_no_crash

#[cfg(test)]
#[test]
fn type_infer_type_instantiations_typeof_local_in_type_pack_no_crash() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _semantics = ScopedFastFlag::new(&FFlag::LuauExplicitTypeInstantiationSupport, true);
    let _visit_call_type_args = ScopedFastFlag::new(&FFlag::LuauVisitCallTypeArgsInDfg, true);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}
        function t:f<T...>() end

        local x = 5

        t:f<<(string, typeof(x))>>()
    "#,
        ),
        None,
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
}
