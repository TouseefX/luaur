//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.oop.test.cpp:142:type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method IrAssemblyFixture::lower (tests/IrAssembly.test.cpp)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory

#[cfg(test)]
#[test]
fn type_infer_oop_inferring_hundreds_of_self_calls_should_not_suffocate_memory() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let _result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        ("foo")
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
            :lower()
    "#,
        ),
        None,
    );

    let module = fixture.get_main_module(false);
    let type_count = unsafe { (*module).internal_types.types.size() };
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(80 >= type_count, "type count was {}", type_count);
    } else {
        assert!(50 >= type_count, "type count was {}", type_count);
    }
}
