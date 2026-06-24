//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:640:type_infer_functions_complicated_return_types_require_an_explicit_annotation`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_functions_complicated_return_types_require_an_explicit_annotation

#[cfg(test)]
#[test]
fn type_infer_functions_complicated_return_types_require_an_explicit_annotation() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local i = 0
        function most_of_the_natural_numbers(): number?
            if i < 10 then
                i += 1
                return i
            else
                return nil
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture.require_type_string(&String::from("most_of_the_natural_numbers"));
    let function_type = unsafe { get_type_id::<FunctionType>(ty).as_ref() }
        .unwrap_or_else(|| panic!("expected function but got {}", to_string_type_id(ty)));

    let ret_type = first(function_type.ret_types(), false).expect("expected return type");
    assert!(!unsafe { get_type_id::<UnionType>(ret_type) }.is_null());
}
