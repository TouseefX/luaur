//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:979:type_infer_extern_types_extern_type_check_present_key_in_superclass`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_extern_types_extern_type_check_present_key_in_superclass

#[cfg(test)]
#[test]
fn type_infer_extern_types_extern_type_check_present_key_in_superclass() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    fixture.load_definition(
        &String::from(
            r#"
        declare extern type FoobarParent with
            IsEnabled: boolean
        end
        declare extern type Foobar extends FoobarParent with
            function Disable(self): ()
        end
    "#,
        ),
        false,
    );

    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local isUsingGamepad = false
        local isModalVisible = false

        local function updateGamepadCursor(foo: Foobar)
            local shouldEnableCursor = isUsingGamepad and isModalVisible

            if foo.IsEnabled == shouldEnableCursor then
                return
            end

            if not shouldEnableCursor then
                foo:Disable()
            end
        end
    "#,
        ),
        None,
    );

    assert!(results.errors.is_empty(), "{:?}", results.errors);
}
