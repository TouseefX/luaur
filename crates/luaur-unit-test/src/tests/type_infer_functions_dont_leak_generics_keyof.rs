//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:4108:type_infer_functions_dont_leak_generics_keyof`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_functions_dont_leak_generics_keyof

#[cfg(test)]
#[test]
fn type_infer_functions_dont_leak_generics_keyof() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function makeOtherThing(template)
            return {
                Stuff = template
            }
        end

        local function makeThing(tbl)
            local returnThis = { Input = makeOtherThing(tbl) }

            function returnThis.Test(key: keyof<typeof(returnThis.Input.Stuff)>) end

            return returnThis
        end

        local thing = makeThing({a=1})
        thing.Test("a")

        local otherthing = makeThing({b = 42, c = 13})
        otherthing.Test("b")
        otherthing.Test("c")
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "{ Input: { Stuff: { a: number } }, Test: (\"a\") -> () }",
        to_string_type_id(fixture.base.require_type_string(&String::from("thing")))
    );
    assert_eq!(
        "{ Input: { Stuff: { b: number, c: number } }, Test: (\"b\" | \"c\") -> () }",
        to_string_type_id(
            fixture
                .base
                .require_type_string(&String::from("otherthing"))
        )
    );
}
