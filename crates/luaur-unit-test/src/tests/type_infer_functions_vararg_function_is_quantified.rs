//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:203:type_infer_functions_vararg_function_is_quantified`
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
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - calls -> method Fixture::getMainModule (tests/Fixture.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_infer_functions_vararg_function_is_quantified

#[cfg(test)]
#[test]
fn type_infer_functions_vararg_function_is_quantified() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r##"
        local T = {}
        function T.f(...)
            local result = {}

            for i = 1, select("#", ...) do
                local dictionary = select(i, ...)
                for key, value in pairs(dictionary) do
                    result[key] = value
                end
            end

            return result
        end

        return T
    "##,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.base.get_main_module(false) };
    let return_ty = first(module.return_type, true).expect("expected module return type");
    let table = unsafe { get_type_id::<TableType>(return_ty).as_ref() }.expect("expected table");
    let f = table.props.get("f").expect("expected f property");
    assert!(f.read_ty.is_some(), "expected readable f property");
}
