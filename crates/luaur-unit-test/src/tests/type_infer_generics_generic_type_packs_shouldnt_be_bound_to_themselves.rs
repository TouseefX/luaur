//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:1836:type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves

#[cfg(test)]
#[test]
fn type_infer_generics_generic_type_packs_shouldnt_be_bound_to_themselves() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
export type t1<T...> = {
    foo: (self: t1<T...>, bar: (T...) -> ()) -> ()
}

export type t2<T...> = {
    baz: (self: t2<T...>) -> t1<T...>,
}

export type t3<T...> = {
    f: (self: t3<T...>, T...)->  (),
    g: t1<T...>,
    h: t1<(Player, T...)>
}

local t2 = {}

function t2.new<T...>(): t2<T...>
end

local function create_t3<T...>(): t3<T...>
    local t2_1 = t2.new()
    local t2_2 = t2.new()
    local my_t3 = {
        f = function(_self: t3<T...>, ...: T...) end,
        g = t2_1:baz(),
        h = t2_2:baz()
    }
    return my_t3
end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}
