//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:2997:type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved

#[cfg(test)]
#[test]
fn type_infer_functions_unpack_depends_on_rhs_pack_to_be_fully_resolved() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
local function id(x)
    return x
end
local u,v = id(3), id(id(44))
"#,
        ),
        None,
    );

    let v_type = fixture.require_type_string(&String::from("v"));
    let number_type = fixture.get_builtins().numberType;
    assert_eq!(number_type, v_type);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
