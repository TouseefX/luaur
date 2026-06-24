//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:843:type_infer_singletons_cli_184125`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_singletons_cli_184125

#[cfg(test)]
#[test]
fn type_infer_singletons_cli_184125() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyTypeA = {Value: true}
        type MyTypeB = {Value: false}

        local function Func(input: number) : (MyTypeA | MyTypeB)
            if input == 1 then
                return {Value = true}
            else
                return {Value = false}
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
