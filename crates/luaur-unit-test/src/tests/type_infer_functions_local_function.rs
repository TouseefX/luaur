//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:516:type_infer_functions_local_function`
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
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - calls -> method Fixture::getPrimitiveType (tests/Fixture.cpp)
//!   - translates_to -> rust_item type_infer_functions_local_function

#[cfg(test)]
#[test]
fn type_infer_functions_local_function() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f()
            return 8
        end

        function g()
            local function f()
                return 'hello'
            end
            return f
        end

        local h = g()
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let h = unsafe { follow_type_id(fixture.require_type_string(&String::from("h"))) };
    let ftv = unsafe { get_type_id::<FunctionType>(h).as_ref() }.expect("expected FunctionType");
    let ret = first(ftv.ret_types(), true).expect("expected return type");
    let ret = unsafe { follow_type_id(ret) };
    assert_eq!("string", to_string_type_id(ret));
}
