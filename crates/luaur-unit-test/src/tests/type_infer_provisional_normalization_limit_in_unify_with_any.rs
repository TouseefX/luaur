//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1102:type_infer_provisional_normalization_limit_in_unify_with_any`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> type_alias ScopedFastInt (tests/ScopedFlags.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function formatAppend (Common/src/StringUtils.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_provisional_normalization_limit_in_unify_with_any

#[cfg(test)]
#[test]
fn type_infer_provisional_normalization_limit_in_unify_with_any() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_common::{FFlag, FInt};

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _normalize_cache_limit = ScopedFastInt::new(&FInt::LuauNormalizeCacheLimit, 1000);

    let parts = 100;
    let mut source = String::new();
    for i in 0..parts {
        source.push_str(&alloc::format!("type T{i} = {{ f{i}: number }}\n"));
    }

    source.push_str("type Instance = { new: (('s0', extra: Instance?) -> T0)");
    for i in 1..parts {
        source.push_str(&alloc::format!(" & (('s{i}', extra: Instance?) -> T{i})"));
    }
    source.push_str(" }\n");

    source.push_str(
        r#"
local Instance: Instance = {} :: any

local function foo(a: typeof(Instance.new)) return if a then 2 else 3 end

foo(1 :: any)
"#,
    );

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture
        .base
        .check_string_optional_frontend_options(&source, None);

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
