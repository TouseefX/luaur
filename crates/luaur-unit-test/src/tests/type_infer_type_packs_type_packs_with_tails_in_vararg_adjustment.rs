//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.typePacks.test.cpp:1085:type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment`
//! Source: `tests/TypeInfer.typePacks.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.typePacks.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.typePacks.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment

#[cfg(test)]
#[test]
fn type_infer_type_packs_type_packs_with_tails_in_vararg_adjustment() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _sff = if !FFlag::DebugLuauForceOldSolver.get() {
        Some(ScopedFastFlag::new(
            &FFlag::LuauInstantiateInSubtyping,
            true,
        ))
    } else {
        None
    };

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function wrapReject<TArg, TResult>(fn: (self: any, ...TArg) -> ...TResult): (self: any, ...TArg) -> ...TResult
            return function(self, ...)
                local arguments = { ... }
                local ok, result = pcall(function()
                    return fn(self, table.unpack(arguments))
                end)
                return result
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
