//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:2157:type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing

#[cfg(test)]
#[test]
fn type_infer_generics_cli_185450_instantiate_generics_prior_to_pushing() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _instantiate_before_push =
        ScopedFastFlag::new(&FFlag::LuauInstantiateFunctionTypeBeforePush, true);

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type Parent = {
            Func1:<P...> (self: Parent, value: boolean, P...) -> (Parent?),
            Func2: (self: Parent, value: boolean) -> (Parent?),
        }

        export type Child = {
            Parent: Parent,
            Func: (self: Child) -> (Child?),
        }

        local Parent = {} :: Parent
        local Child = {} :: Child

        function Parent:Func1(value, ...)
            if value then return self else return nil end
        end

        function Parent:Func2(value)
            if value then return self else return nil end
        end

        function Child:Func()
            if math.random() > 0.5 then return self else return nil end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
