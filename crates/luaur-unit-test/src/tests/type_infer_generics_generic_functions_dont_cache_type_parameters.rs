//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.generics.test.cpp:873:type_infer_generics_generic_functions_dont_cache_type_parameters`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_generics_generic_functions_dont_cache_type_parameters

#[cfg(test)]
#[test]
fn type_infer_generics_generic_functions_dont_cache_type_parameters() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
-- See https://github.com/luau-lang/luau/issues/332
-- This function has a type parameter with the same name as clones,
-- so if we cache type parameter names for functions these get confused.
-- function id<Z>(x : Z) : Z
function id<X>(x : X) : X
  return x
end

function clone<X, Y>(dict: {[X]:Y}): {[X]:Y}
  local copy = {}
  for k, v in pairs(dict) do
    copy[k] = v
  end
  return copy
end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
